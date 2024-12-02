use embedded_hal::{
    delay::DelayNs,
    spi::{ErrorKind, ErrorType, Operation, SpiBus, SpiDevice},
};
use riscv::interrupt;

use super::{PinCS, PinsFull, PinsNoCS, SharedBus, SpiConfig, SpiX};

/// SPI shared device abstraction
pub struct SpiSharedDevice<'bus, SPI, PINS, CS, D> {
    bus: &'bus SharedBus<SPI, PINS>,
    cs: CS,
    config: SpiConfig,
    delay: D,
}

impl<SPI, PINS, CS, D> SpiSharedDevice<'_, SPI, PINS, CS, D> {
    /// Releases the CS pin and delay back
    pub fn release(self) -> (CS, D) {
        (self.cs, self.delay)
    }
}

impl<'bus, SPI, PINS, CS, D> SpiSharedDevice<'bus, SPI, PINS, CS, D>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
    D: DelayNs,
{
    /// Create shared [SpiSharedDevice] using the existing [SharedBus]
    /// and given [SpiConfig]. The config gets cloned.
    pub fn new(bus: &'bus SharedBus<SPI, PINS>, cs: CS, config: &SpiConfig, delay: D) -> Self {
        Self {
            bus,
            cs,
            config: config.clone(),
            delay,
        }
    }
}

impl<SPI, PINS, CS, D> ErrorType for SpiSharedDevice<'_, SPI, PINS, CS, D>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
    D: DelayNs,
{
    type Error = ErrorKind;
}

impl<SPI, PINS, CS, D> SpiDevice for SpiSharedDevice<'_, SPI, PINS, CS, D>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI> + PinsFull<SPI>,
    CS: PinCS<SPI>,
    D: DelayNs,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        let mut bus =
            interrupt::free(|| self.bus.try_borrow_mut().map_err(|_| ErrorKind::ModeFault))?;
        // Safety: valid CS index
        unsafe { bus.configure(&self.config, Some(CS::CS_INDEX)) };
        bus.start_frame();

        let mut res = Ok(());
        for operation in operations.iter_mut() {
            res = match operation {
                Operation::Read(read) => bus.read(read),
                Operation::Write(write) => bus.write(write),
                Operation::Transfer(read, write) => bus.transfer(read, write),
                Operation::TransferInPlace(read_write) => bus.transfer_in_place(read_write),
                Operation::DelayNs(ns) => {
                    self.delay.delay_ns(*ns);
                    Ok(())
                }
            };
            if res.is_err() {
                break;
            }
        }

        if res.is_ok() {
            bus.flush()?;
        }
        bus.end_frame();

        Ok(())
    }
}
