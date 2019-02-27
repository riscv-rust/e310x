//! Clock configuration
use e310x::{prci, PRCI, AONCLK};
use clint::{MCYCLE, MTIME};
use riscv::interrupt;
use time::Hertz;

/// const BOARD_HFXOSC_FREQ: u32
/// const BOARD_LFALTCLK_FREQ: u32
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

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
        if cfg!(feature = "hfxosc") {
            CoreClk {
                hfxosc: true,
                pll: false,
                freq: Hertz(BOARD_HFXOSC_FREQ),
            }
        } else {
            CoreClk {
                hfxosc: false,
                pll: false,
                // Default after reset
                freq: Hertz(13_800_000),
            }
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

/// Constrainted PRCI peripheral
pub struct CoreClk {
    hfxosc: bool,
    pll: bool,
    freq: Hertz,
}

impl CoreClk {
    /// Use external clock. Requires feature hfxosc and BOARD_HFXOSC_FREQ
    /// should be set at build time if the external oscillator is not 16MHz.
    #[cfg(feature = "hfxosc")]
    pub fn use_external(mut self) -> Self {
        self.hfxosc = true;
        self.freq = Hertz(BOARD_HFXOSC_FREQ);
        self
    }

    /// Use internal clock. Sets frequency to 13.8MHz.
    pub fn use_internal(mut self) -> Self {
        self.hfxosc = false;
        self.freq = Hertz(13_800_000);
        self
    }

    /// Use pll. Sets frequency to 256MHz. Requires feature pll.
    /// NOTE: Assumes an external 16MHz oscillator is available.
    #[cfg(feature = "pll")]
    pub fn use_pll(mut self) -> Self {
        self.pll = true;
        self.freq = Hertz(256_000_000);
        self
    }

    /// Freezes the clock frequencies.
    pub(crate) fn freeze(mut self) -> Hertz {
        if self.pll {
            unsafe { self.use_hfpll(); }
        } else if self.hfxosc {
            unsafe { self.use_hfxosc(); }
        } else {
            unsafe { self.use_hfrosc(); }
        }

        self.freq
    }

    /// Use internal oscillator with bypassed pll.
    unsafe fn use_hfrosc(&mut self) {
        let prci = &*PRCI::ptr();

        // Enable HFROSC
        prci.hfrosccfg.write(|w| {
            w.enable().bit(true)
            // It is OK to change this even if we are running off of it.
            // Reset them to default values. (13.8MHz)
                .div().bits(4)
                .trim().bits(16)
        });
        // Wait for HFROSC to stabilize
        while !prci.hfrosccfg.read().ready().bit_is_set() {}
        // Switch to HFROSC
        prci.pllcfg.modify(|_, w| {
            w.sel().bit(false)
        });
        // Bypass PLL to save power
        prci.pllcfg.modify(|_, w| {
            w.bypass().bit(true)
            // Select HFROSC as PLL ref to disable HFXOSC later
                .refsel().bit(false)
        });
        // Disable HFXOSC to save power.
        prci.hfxosccfg.write(|w| w.enable().bit(false));
    }

    /// Use external oscillator with bypassed pll.
    unsafe fn use_hfxosc(&mut self) {
        let prci = &*PRCI::ptr();

        self.init_pll(|_, w| {
            // bypass PLL
            w.bypass().bit(true)
            // select HFXOSC
                .refsel().bit(true)
        }, |w| w.divby1().bit(true));
        // Disable HFROSC to save power
        prci.hfrosccfg.write(|w| w.enable().bit(false));
    }

    /// Use external oscillator with pll. Sets PLL
    /// r=2, f=64, q=2 values to maximum allowable
    /// for a 16MHz reference clock. Output frequency
    /// is 16MHz / 2 * 64 / 2 = 256MHz.
    /// NOTE: By trimming the internal clock to 12MHz
    /// and using r=1, f=64, q=2 the maximum frequency
    /// of 384MHz can be reached.
    unsafe fn use_hfpll(&mut self) {
        let prci = &*PRCI::ptr();

        self.init_pll(|_, w| {
            // bypass PLL
            w.bypass().bit(false)
            // select HFXOSC
                .refsel().bit(true)
            // bits = r - 1
                .pllr().bits(1)
            // bits = f / 2 - 1
                .pllf().bits(31)
            // bits = q=2 -> 1, q=4 -> 2, q=8 -> 3
                .pllq().bits(1)
        }, |w| w.divby1().bit(true));
        // Disable HFROSC to save power
        prci.hfrosccfg.write(|w| w.enable().bit(false));
    }

    /*
    /// Compute PLL multiplier.
    fn pll_mult(&self) -> u32 {
        let prci = unsafe { &*PRCI::ptr() };

        let pllcfg = prci.pllcfg.read();
        let plloutdiv = prci.plloutdiv.read();

        let r = pllcfg.pllr().bits() as u32 + 1;
        let f = (pllcfg.pllf().bits() as u32 + 1) * 2;
        let q = [2, 4, 8][pllcfg.pllq().bits() as usize - 1];

        let div = match plloutdiv.divby1().bit() {
            true => 1,
            false => (plloutdiv.div().bits() as u32 + 1) * 2,
        };

        f / r / q / div
    }*/

    /// Wait for the pll to lock.
    fn wait_for_lock(&self) {
        let prci = unsafe { &*PRCI::ptr() };
        // NOTE: reading mtime should always be safe.
        let mtime = MTIME;

        // Won't lock when bypassed and will loop forever
        if !prci.pllcfg.read().bypass().bit_is_set() {
            // Wait for PLL Lock
            // Note that the Lock signal can be glitchy.
            // Need to wait 100 us
            // RTC is running at 32kHz.
            // So wait 4 ticks of RTC.
            let time = mtime.mtime() + 4;
            while mtime.mtime() < time {}
            // Now it is safe to check for PLL Lock
            while !prci.pllcfg.read().lock().bit_is_set() {}
        }
    }

    unsafe fn init_pll<F, G>(&mut self, pllcfg: F, plloutdiv: G)
        where
        for<'w> F: FnOnce(&prci::pllcfg::R,
                          &'w mut prci::pllcfg::W) -> &'w mut prci::pllcfg::W,
    for<'w> G: FnOnce(&'w mut prci::plloutdiv::W) -> &'w mut prci::plloutdiv::W,
    {
        let prci = &*PRCI::ptr();
        // Make sure we are running of internal clock
        // before configuring the PLL.
        self.use_hfrosc();
        // Enable HFXOSC
        prci.hfxosccfg.write(|w| w.enable().bit(true));
        // Wait for HFXOSC to stabilize
        while !prci.hfxosccfg.read().ready().bit_is_set() {}
        // Configure PLL
        prci.pllcfg.modify(pllcfg);
        prci.plloutdiv.write(plloutdiv);
        // Wait for PLL lock
        self.wait_for_lock();
        // Switch to PLL
        prci.pllcfg.modify(|_, w| {
            w.sel().bit(true)
        });
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
