use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiBus, SpiDevice};

use crate::spi::SpiConfig;

use super::{Pins, PinsFull, SpiBus as Bus, SpiX};

/// SPI exclusive device abstraction
pub struct SpiExclusiveDevice<SPI, PINS> {
    bus: Bus<SPI, PINS>,
}

impl<SPI: SpiX, PINS: Pins<SPI>> SpiExclusiveDevice<SPI, PINS> {
    /// Create [`SpiExclusiveDevice`] using existing [`SpiBus`](Bus) with the given [`SpiConfig`]
    pub fn new(mut bus: Bus<SPI, PINS>, config: &SpiConfig) -> Self {
        // Safety: valid CS index
        unsafe { bus.configure(config, PINS::CS_INDEX) };

        Self { bus }
    }

    /// Releases the Bus back deconstructing it
    pub fn release(self) -> (SPI, PINS) {
        self.bus.release()
    }
}

impl<SPI: SpiX, PINS: Pins<SPI>> ErrorType for SpiExclusiveDevice<SPI, PINS> {
    type Error = ErrorKind;
}

impl<SPI: SpiX, PINS: PinsFull<SPI>> SpiDevice for SpiExclusiveDevice<SPI, PINS> {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.bus.start_frame();

        let mut res = Ok(());
        for operation in operations.iter_mut() {
            res = match operation {
                Operation::Read(read) => self.bus.read(read),
                Operation::Write(write) => self.bus.write(write),
                Operation::Transfer(read, write) => self.bus.transfer(read, write),
                Operation::TransferInPlace(read_write) => self.bus.transfer_in_place(read_write),
                Operation::DelayNs(_ns) => todo!(),
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
