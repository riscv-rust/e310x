use embedded_hal::spi::{
    blocking::{Operation, Transactional, Transfer, TransferInplace, Write, WriteIter},
    nb::FullDuplex,
    ErrorKind, ErrorType,
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

impl<SPI, PINS> ErrorType for SpiExclusiveDevice<SPI, PINS> {
    type Error = ErrorKind;
}

impl<SPI, PINS> FullDuplex for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.bus.read()
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.bus.send(byte)
    }
}

impl<SPI, PINS> Transfer for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.bus.start_frame();
        let result = self.bus.transfer(read, write);
        self.bus.end_frame();

        result
    }
}

impl<SPI, PINS> TransferInplace for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    fn transfer_inplace<'w>(&mut self, words: &'w mut [u8]) -> Result<(), Self::Error> {
        self.bus.start_frame();
        let result = self.bus.transfer_inplace(words);
        self.bus.end_frame();

        result
    }
}

impl<SPI, PINS> Write for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.bus.start_frame();
        let result = self.bus.transfer(&mut [], words);
        self.bus.end_frame();

        result
    }
}

impl<SPI, PINS> WriteIter for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        self.bus.start_frame();
        let result = self.bus.write_iter(words);
        self.bus.end_frame();

        result
    }
}

impl<SPI, PINS> Transactional for SpiExclusiveDevice<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    fn exec<'op>(&mut self, operations: &mut [Operation<'op, u8>]) -> Result<(), Self::Error> {
        self.bus.start_frame();
        let result = self.bus.exec(operations);
        self.bus.end_frame();

        result
    }
}
