use core::convert::Infallible;

use embedded_hal::{
    blocking::spi::{Transfer, Write, WriteIter},
    spi::FullDuplex,
};

use crate::spi::SpiConfig;

use super::{Pins, SpiBus, SpiX};

/// SPI exclusive device abstraction
pub struct SpiExclusiveDevice<SPI, PINS> {
    bus: SpiBus<SPI, PINS>,
}

impl<SPI, PINS> SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    /// Create [SpiExclusiveDevice] using the existing [SpiBus](super::SpiBus)
    /// with the given [SpiConfig]
    pub fn new(mut bus: SpiBus<SPI, PINS>, config: &SpiConfig) -> Self
    where
        PINS: Pins<SPI>,
    {
        bus.configure(config, PINS::CS_INDEX);

        Self { bus }
    }

    /// Releases the Bus back deconstructing it
    pub fn release(self) -> (SPI, PINS) {
        self.bus.release()
    }
}

impl<SPI, PINS> FullDuplex<u8> for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    type Error = Infallible;

    fn try_read(&mut self) -> nb::Result<u8, Infallible> {
        self.bus.read()
    }

    fn try_send(&mut self, byte: u8) -> nb::Result<(), Infallible> {
        self.bus.send(byte)
    }
}

impl<SPI, PINS> Transfer<u8> for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    type Error = Infallible;

    fn try_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.bus.start_frame();
        let result = self.bus.transfer(words);
        self.bus.end_frame();

        result
    }
}

impl<SPI, PINS> Write<u8> for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    type Error = Infallible;

    fn try_write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.bus.start_frame();
        let result = self.bus.write(words);
        self.bus.end_frame();

        result
    }
}

impl<SPI, PINS> WriteIter<u8> for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    type Error = Infallible;

    fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        self.bus.start_frame();
        let result = self.bus.write_iter(words);
        self.bus.end_frame();

        result
    }
}
