//! Clock configuration
use e310x::{PRCI, AONCLK};
use clint::{MCYCLE, MTIME};
use riscv::interrupt;
use time::Hertz;


const PLLREF_MIN: u32 = 6_000_000;
const PLLREF_MAX: u32 = 48_000_000;
const REFR_MIN: u32 = 6_000_000;
const REFR_MAX: u32 = 12_000_000;
const VCO_MIN: u32 = 384_000_000;
const VCO_MAX: u32 = 768_000_000;
const PLLOUT_MIN: u32 = 48_000_000;
const PLLOUT_MAX: u32 = 384_000_000;
const DIVOUT_MIN: u32 = 375_000;
const DIVOUT_MAX: u32 = 384_000_000;


/// PrciExt trait extends `PRCI` peripheral.
pub trait PrciExt {
    /// Constrains the `PRCI` peripheral so it plays nicely with the other
    /// abstractions.
    fn constrain(self) -> CoreClk;
}

/// AonExt trait extends `AONCLK` peripheral.
pub trait AonExt {
    /// Constrains the `AON` peripheral so it plays nicely with the other
    /// abstractions.
    fn constrain(self) -> AonClk;
}

impl PrciExt for PRCI {
    fn constrain(self) -> CoreClk {
        CoreClk {
            hfxosc: None,
            coreclk: Hertz(13_800_000), // Default after reset
        }
    }
}

impl AonExt for AONCLK {
    fn constrain(self) -> AonClk {
        AonClk {
            lfaltclk: None,
        }
    }
}

/// Constrainted `PRCI` peripheral
pub struct CoreClk {
    hfxosc: Option<Hertz>,
    coreclk: Hertz,
}

impl CoreClk {
    /// Uses `HFXOSC` (external oscillator) instead of `HFROSC` (internal ring oscillator) as the clock source.
    pub fn use_external<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let hz: Hertz = freq.into();
        assert!(hz.0 < 20_000_000);

        self.hfxosc = Some(hz);
        self
    }

    /// Sets the desired frequency for the `coreclk` clock
    pub fn coreclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
        self.coreclk = freq.into();
        self
    }

    /// Freezes high-frequency clock configuration, making it effective
    pub(crate) fn freeze(self) -> Hertz {
        // Assume `psdclkbypass_n` is not used

        if let Some(freq) = self.hfxosc {
            self.configure_with_external(freq)
        } else {
            self.configure_with_internal()
        }
    }

    /// Configures clock generation system with external oscillator
    fn configure_with_external(self, source_freq: Hertz) -> Hertz {
        let prci = unsafe { &*PRCI::ptr() };

        // Enable HFXOSC
        prci.hfxosccfg.write(|w| w.enable().bit(true));

        // Wait for HFXOSC to stabilize
        while !prci.hfxosccfg.read().ready().bit_is_set() {}

        // Select HFXOSC as pllref
        prci.pllcfg.modify(|_, w| w.refsel().bit(true));

        let freq;
        if source_freq.0 == self.coreclk.0 {
            // Use external oscillator with bypassed PLL
            freq = source_freq;

            // Bypass PLL
            prci.pllcfg.modify(|_, w| w.bypass().bit(true));

            // Bypass divider
            prci.plloutdiv.write(|w| w.divby1().bit(true));
        } else {
            // Use external oscillator with PLL

            // Configure PLL and divider
            freq = self.configure_pll(source_freq, self.coreclk);
        }

        // Switch to PLL
        prci.pllcfg.modify(|_, w| w.sel().bit(true));

        // Disable HFROSC to save power
        prci.hfrosccfg.write(|w| w.enable().bit(false));

        freq
    }

    /// Configures clock generation system with internal oscillator
    fn configure_with_internal(self) -> Hertz {
        let prci = unsafe { &*PRCI::ptr() };

        let hfrosc_freq = self.configure_hfrosc();

        let freq;
        if hfrosc_freq.0 == self.coreclk.0 {
            // Use internal oscillator with bypassed PLL
            freq = hfrosc_freq;

            // Switch to HFROSC, bypass PLL to save power
            prci.pllcfg.modify(|_, w| w
                .sel().bit(false)
                .bypass().bit(true)
            );

            //
            prci.pllcfg.modify(|_, w| w.bypass().bit(true));
        } else {
            // Use internal oscillator with PLL

            // Configure PLL and divider
            freq = self.configure_pll(hfrosc_freq, self.coreclk);

            // Switch to PLL
            prci.pllcfg.modify(|_, w| w.sel().bit(true));

        }

        // Disable HFXOSC to save power
        prci.hfxosccfg.write(|w| w.enable().bit(false));

        freq
    }

    /// Configures internal high-frequency oscillator (`HFROSC`)
    fn configure_hfrosc(&self) -> Hertz {
        let prci = unsafe { &*PRCI::ptr() };

        // TODO: use trim value from OTP

        // Configure HFROSC to 13.8 MHz
        prci.hfrosccfg.write(|w| unsafe { w
            .div().bits(4)
            .trim().bits(16)
            .enable().bit(true)
        });

        // Wait for HFROSC to stabilize
        while !prci.hfrosccfg.read().ready().bit_is_set() {}

        Hertz(13_800_000)
    }

    /// Configures PLL and PLL Output Divider
    /// The resulting frequency may differ by 0-2% from the requested
    fn configure_pll(&self, pllref_freq: Hertz, divout_freq: Hertz) -> Hertz {
        let pllref_freq = pllref_freq.0;
        assert!(PLLREF_MIN <= pllref_freq && pllref_freq <= PLLREF_MAX);

        let divout_freq = divout_freq.0;
        assert!(DIVOUT_MIN <= divout_freq && divout_freq <= DIVOUT_MAX);

        // Calculate PLL Output Divider settings
        let divider_div;
        let divider_bypass;

        let d = PLLOUT_MAX / divout_freq;
        if d > 1 {
            divider_bypass = false;

            if d > 128 {
                divider_div = (128 / 2) - 1;
            } else {
                divider_div = (d / 2) - 1;
            }
        } else {
            divider_div = 0;
            divider_bypass = true;
        }

        // Calculate pllout frequency
        let d = if divider_bypass {
            1
        } else {
            2 * (divider_div + 1)
        };
        let pllout_freq = divout_freq * d;
        assert!(PLLOUT_MIN <= pllout_freq && pllout_freq <= PLLOUT_MAX);

        // Calculate PLL R ratio
        let r = match pllref_freq {
            24_000_000...48_000_000 => 4,
            18_000_000...24_000_000 => 3,
            12_000_000...18_000_000 => 2,
             6_000_000...12_000_000 => 1,
            _ => unreachable!(),
        };

        // Calculate refr frequency
        let refr_freq = pllref_freq / r;
        assert!(REFR_MIN <= refr_freq && refr_freq <= REFR_MAX);

        // Calculate PLL Q ratio
        let q = match pllout_freq {
            192_000_000...384_000_000 => 2,
             96_000_000...192_000_000 => 4,
             48_000_000...96_000_000 => 8,
            _ => unreachable!(),
        };

        // Calculate the desired vco frequency
        let target_vco_freq = pllout_freq * q;
        assert!(VCO_MIN <= target_vco_freq && target_vco_freq <= VCO_MAX);

        // Calculate PLL F ratio
        let f = target_vco_freq / refr_freq;
        assert!(f <= 128);

        // Choose the best F ratio
        let f_lo = (f / 2) * 2; // F must be a multiple of 2
        let vco_lo = refr_freq * f_lo;
        let f_hi = f_lo + 2;
        let vco_hi = refr_freq * f_hi;
        let (f, vco_freq) = if (f_hi <= 128 && vco_hi <= VCO_MAX) && (target_vco_freq as i32 - vco_hi as i32).abs() < (target_vco_freq as i32 - vco_lo as i32).abs() {
            (f_hi, vco_hi)
        } else {
            (f_lo, vco_lo)
        };
        assert!(VCO_MIN <= vco_freq && vco_freq <= VCO_MAX);

        // Calculate actual pllout frequency
        let pllout_freq = vco_freq / q;
        assert!(PLLOUT_MIN <= pllout_freq && pllout_freq <= PLLOUT_MAX);

        // Calculate actual divout frequency
        let divout_freq = pllout_freq / d;
        assert!(DIVOUT_MIN <= divout_freq && divout_freq <= DIVOUT_MAX);

        // Calculate bit-values
        let r: u8 = (r - 1) as u8;
        let f: u8 = (f / 2 - 1) as u8;
        let q: u8 = match q {
            2 => 0b01,
            4 => 0b10,
            8 => 0b11,
            _ => unreachable!(),
        };

        // Configure PLL
        let prci = unsafe { &*PRCI::ptr() };
        prci.pllcfg.modify(|_, w| unsafe { w
            .pllr().bits(r)
            .pllf().bits(f)
            .pllq().bits(q)
            .bypass().bit(false)
        });

        // Configure PLL Output Divider
        prci.plloutdiv.write(|w| unsafe { w
            .div().bits(divider_div as u8)
            .divby1().bit(divider_bypass)
        });

        // Wait for PLL Lock
        // Note that the Lock signal can be glitchy.
        // Need to wait 100 us
        // RTC is running at 32kHz.
        // So wait 4 ticks of RTC.
        let mtime = MTIME;
        let time = mtime.mtime() + 4;
        while mtime.mtime() < time {}
        // Now it is safe to check for PLL Lock
        while !prci.pllcfg.read().lock().bit_is_set() {}

        Hertz(divout_freq)
    }
}

/// Constrained `AONCLK` peripheral
pub struct AonClk {
    lfaltclk: Option<Hertz>,
}

impl AonClk {
    /// Uses `LFALTCLK` (external low-frequency clock) instead of `LFROSC` (internal ring oscillator) as the clock source.
    pub fn use_external<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let hz: Hertz = freq.into();
        assert!(hz.0 < 500_000);

        self.lfaltclk = Some(hz);
        self
    }

    /// Freezes low-frequency clock configuration, making it effective
    pub(crate) fn freeze(self) -> Hertz {
        let aonclk = unsafe { &*AONCLK::ptr() };

        if let Some(freq) = self.lfaltclk {
            // Use external oscillator.

            // Disable unused LFROSC to save power.
            aonclk.lfrosccfg.write(|w| w.enable().bit(false));

            freq
        } else {
            // Use internal oscillator.

            let trim = 16;
            let div = 4; // LFROSC/5

            // Configure LFROSC
            aonclk.lfrosccfg.write(|w| {
                unsafe {
                    w.bits(trim << 16) // TODO: replace this with trim()
                     .div().bits(div)
                     .enable().bit(true)
                }
            });

            // Wait for LFROSC to stabilize
            while !aonclk.lfrosccfg.read().ready().bit_is_set() {}

            Hertz(32_768) // It's not so accurate: ≈30 kHz according to the datasheet
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no
/// longer be changed.
#[derive(Clone, Copy)]
pub struct Clocks {
    coreclk: Hertz,
    lfclk: Hertz,
}

impl Clocks {
    /// Freezes the coreclk and aonclk frequencies.
    pub fn freeze(coreclk: CoreClk, aonclk: AonClk) -> Self {
        let coreclk = coreclk.freeze();
        let lfclk = aonclk.freeze();
        Clocks { coreclk, lfclk }
    }

    /// Returns the frozen coreclk frequency
    pub fn coreclk(&self) -> Hertz {
        self.coreclk
    }

    /// Returns the frozen lfclk frequency
    pub fn lfclk(&self) -> Hertz {
        self.lfclk
    }

    /// Measure the coreclk frequency by counting the number of aonclk ticks.
    fn _measure_coreclk(&self, min_ticks: u64, mcycle: &MCYCLE) -> Hertz {
        let mtime = MTIME;
        interrupt::free(|_| {
            // Don't start measuring until we see an mtime tick
            while mtime.mtime() == mtime.mtime() {}

            let start_cycle = mcycle.mcycle();
            let start_time = mtime.mtime();

            // Wait for min_ticks to pass
            while start_time + min_ticks > mtime.mtime() {}

            let end_cycle = mcycle.mcycle();
            let end_time = mtime.mtime();

            let delta_cycle: u64 = end_cycle - start_cycle;
            let delta_time: u64 = end_time - start_time;

            let res = (delta_cycle / delta_time) * 32768
                + ((delta_cycle % delta_time) * 32768) / delta_time;
            // u32 can represent 4GHz way above the expected measurement value
            Hertz(res as u32)
        })
    }

    /// Measure the coreclk frequency by counting the number of aonclk ticks.
    pub fn measure_coreclk(&self, mcycle: &MCYCLE) -> Hertz {
        // warm up I$
        self._measure_coreclk(1, mcycle);
        // measure for real
        self._measure_coreclk(10, mcycle)
    }
}
