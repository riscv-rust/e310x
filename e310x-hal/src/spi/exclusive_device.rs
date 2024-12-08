use embedded_hal::{
    delay::DelayNs,
    spi::{self, ErrorType, Operation, SpiBus, SpiDevice},
};

use crate::spi::SpiConfig;

use super::{Pins, PinsFull, SpiBus as Bus, SpiX};

/// SPI exclusive device abstraction with delay support.
pub struct SpiExclusiveDevice<SPI, PINS, D> {
    bus: Bus<SPI, PINS>,
    delay: D,
}

impl<SPI, PINS, D> SpiExclusiveDevice<SPI, PINS, D>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    D: DelayNs,
{
    /// Create [`SpiDelayedExclusiveDevice`] using existing [`SpiBus`](Bus) with the given [`SpiConfig`]
    pub fn new(mut bus: Bus<SPI, PINS>, config: &SpiConfig, delay: D) -> Self {
        // Safety: valid CS index
        unsafe { bus.configure(config, PINS::CS_INDEX) };

        Self { bus, delay }
    }

    /// Releases the Bus and Delay back deconstructing it
    pub fn release(self) -> (SPI, PINS, D) {
        let (spi, pins) = self.bus.release();
        (spi, pins, self.delay)
    }
}

impl<SPI, PINS, D> ErrorType for SpiExclusiveDevice<SPI, PINS, D>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
    D: DelayNs,
{
    type Error = spi::ErrorKind;
}

impl<SPI, PINS, D> SpiDevice for SpiExclusiveDevice<SPI, PINS, D>
where
    SPI: SpiX,
    PINS: PinsFull<SPI>,
    D: DelayNs,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.bus.start_frame();

        let mut res = Ok(());
        for operation in operations.iter_mut() {
            res = match operation {
                Operation::Read(read) => self.bus.read(read),
                Operation::Write(write) => self.bus.write(write),
                Operation::Transfer(read, write) => self.bus.transfer(read, write),
                Operation::TransferInPlace(read_write) => self.bus.transfer_in_place(read_write),
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
            self.bus.flush()?;
        }
        self.bus.end_frame();
        res
    }
}
