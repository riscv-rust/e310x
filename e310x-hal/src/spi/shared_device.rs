use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiBus, SpiDevice};
use riscv::interrupt;

use super::{PinCS, PinsFull, PinsNoCS, SharedBus, SpiConfig, SpiX};

/// SPI shared device abstraction
pub struct SpiSharedDevice<'bus, SPI, PINS, CS> {
    bus: &'bus SharedBus<SPI, PINS>,
    cs: CS,
    config: SpiConfig,
}

impl<SPI, PINS, CS> SpiSharedDevice<'_, SPI, PINS, CS> {
    /// Releases the CS pin back
    pub fn release(self) -> CS {
        self.cs
    }
}

impl<'bus, SPI, PINS, CS> SpiSharedDevice<'bus, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
{
    /// Create shared [SpiSharedDevice] using the existing [SharedBus]
    /// and given [SpiConfig]. The config gets cloned.
    pub fn new(bus: &'bus SharedBus<SPI, PINS>, cs: CS, config: &SpiConfig) -> Self {
        Self {
            bus,
            cs,
            config: config.clone(),
        }
    }
}

impl<SPI, PINS, CS> ErrorType for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
{
    type Error = ErrorKind;
}

impl<SPI, PINS, CS> SpiDevice for SpiSharedDevice<'_, SPI, PINS, CS>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI> + PinsFull<SPI>,
    CS: PinCS<SPI>,
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
                Operation::DelayNs(_ns) => todo!(),
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
