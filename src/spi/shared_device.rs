use embedded_hal::spi::{
    blocking::{Operation, Transactional, Transfer, TransferInplace, Write, WriteIter},
    nb::FullDuplex,
    ErrorKind, ErrorType,
};
use riscv::interrupt;

use super::{PinCS, Pins, PinsNoCS, SharedBus, SpiConfig, SpiX};

/// SPI shared device abstraction
pub struct SpiSharedDevice<'bus, SPI, PINS, CS> {
    bus: &'bus SharedBus<SPI, PINS>,
    cs: CS,
    config: SpiConfig,
}

impl<'bus, SPI, PINS, CS> SpiSharedDevice<'bus, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
{
    /// Create shared [SpiSharedDevice] using the existing [SharedBus]
    /// and given [SpiConfig]. The config gets cloned.
    pub fn new(bus: &'bus SharedBus<SPI, PINS>, cs: CS, config: &SpiConfig) -> Self
    where
        PINS: PinsNoCS<SPI>,
    {
        Self {
            bus,
            cs,
            config: config.clone(),
        }
    }

    /// Releases the CS pin back
    pub fn release(self) -> CS {
        self.cs
    }
}

impl<SPI, PINS, CS> ErrorType for SpiSharedDevice<'_, SPI, PINS, CS> {
    type Error = ErrorKind;
}

impl<SPI, PINS, CS> FullDuplex for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    CS: PinCS<SPI>,
{
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        interrupt::free(|cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.read()
        })
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        interrupt::free(|cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.send(byte)
        })
    }
}

impl<SPI, PINS, CS> Transfer for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    CS: PinCS<SPI>,
{
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        interrupt::free(move |cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.start_frame();
            let result = bus.transfer(read, write);
            bus.end_frame();

            result
        })
    }
}

impl<SPI, PINS, CS> TransferInplace for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    CS: PinCS<SPI>,
{
    fn transfer_inplace<'w>(&mut self, words: &'w mut [u8]) -> Result<(), Self::Error> {
        interrupt::free(move |cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.start_frame();
            let result = bus.transfer_inplace(words);
            bus.end_frame();

            result
        })
    }
}

impl<SPI, PINS, CS> Write for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    CS: PinCS<SPI>,
{
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        interrupt::free(|cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.start_frame();
            let result = bus.transfer(&mut [], words);
            bus.end_frame();

            result
        })
    }
}

impl<SPI, PINS, CS> WriteIter for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    CS: PinCS<SPI>,
{
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        interrupt::free(|cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.start_frame();
            let result = bus.write_iter(words);
            bus.end_frame();

            result
        })
    }
}

impl<SPI, PINS, CS> Transactional for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    CS: PinCS<SPI>,
{
    fn exec<'op>(&mut self, operations: &mut [Operation<'op, u8>]) -> Result<(), Self::Error> {
        interrupt::free(|cs| {
            let mut bus = self.bus.borrow(*cs).borrow_mut();

            bus.configure(&self.config, Some(CS::CS_INDEX));

            bus.start_frame();
            let result = bus.exec(operations);
            bus.end_frame();

            result
        })
    }
}
