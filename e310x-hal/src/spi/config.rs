use e310x::qspi0::csmode::Mode as CsMode;
use embedded_hal::spi::Mode;

use crate::{clock::Clocks, time::Hertz};

/// SPI Bus configuration

#[derive(Clone)]
/// SPI Bus configuration
pub struct SpiConfig {
    /// SPI Mode
    pub mode: Mode,
    /// Clock Divisor calculated from frozen core clock frequency and SPI frequency
    pub(crate) clock_divisor: u32,
    /// CS Mode
    pub cs_mode: CsMode,
    /// Watermark level for transmits
    pub txmark: u8,
    /// Watermark level for received
    pub rxmark: u8,
    /// Configuration values for CS and SCK related delays
    pub delays: SpiDelayConfig,
}

#[derive(Clone)]
/// Configuration values for CS and SCK related delays
pub struct SpiDelayConfig {
    /// delay between assert and clock in clock ticks
    pub cssck: u8,
    /// delay between clock and de-assert in clock ticks
    pub sckcs: u8,
    /// delay between CS re-assets in clock ticks
    pub intercs: u8,
    /// delay between frames when not re-asserting CS in clock ticks
    pub interxfr: u8,
}

impl SpiConfig {
    /// Create new default configuration with given [Mode] and frequency using core [Clocks]
    pub fn new(mode: Mode, freq: Hertz, clocks: &Clocks) -> Self {
        let clock_divisor = clocks.tlclk().0 / (2 * freq.0) - 1;
        assert!(clock_divisor <= 0xfff);

        Self {
            mode,
            clock_divisor,
            cs_mode: CsMode::Hold,
            txmark: 1,
            rxmark: 0,
            delays: SpiDelayConfig::default(),
        }
    }

    /// Calculated clock divisor
    pub fn clock_divisor(&self) -> u32 {
        self.clock_divisor
    }
}

impl Default for SpiDelayConfig {
    fn default() -> Self {
        Self {
            cssck: 1,    // 1 cycle delay between CS assert and first clock
            sckcs: 1,    // 1 cycle delay between last clock and CS de-assert
            intercs: 1,  // 1 cycle delay between CS re-asserts
            interxfr: 0, // no delay intra-frame when not CS re-asserting
        }
    }
}
