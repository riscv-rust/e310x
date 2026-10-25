use super::SharedBus;
use crate::spi::{PinCS, PinsFull, PinsNoCS, SpiConfig, SpiX};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal_async::{
    delay::DelayNs,
    spi::{ErrorKind, ErrorType, Operation, SpiBus, SpiDevice},
};

/// SPI shared device abstraction
pub struct SpiSharedDevice<'bus, M: RawMutex, SPI, PINS, CS, D> {
    bus: &'bus SharedBus<M, SPI, PINS>,
    cs: CS,
    config: SpiConfig,
    delay: D,
}

impl<M: RawMutex, SPI, PINS, CS, D> SpiSharedDevice<'_, M, SPI, PINS, CS, D> {
    /// Releases the CS pin and delay back
    pub fn release(self) -> (CS, D) {
        (self.cs, self.delay)
    }
}

impl<'bus, M, SPI, PINS, CS, D> SpiSharedDevice<'bus, M, SPI, PINS, CS, D>
where
    M: RawMutex,
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
    D: DelayNs,
{
    /// Create shared [SpiSharedDevice] using the existing [SharedBus]
    /// and given [SpiConfig]. The config gets cloned.
    pub fn new(bus: &'bus SharedBus<M, SPI, PINS>, cs: CS, config: &SpiConfig, delay: D) -> Self {
        Self {
            bus,
            cs,
            config: config.clone(),
            delay,
        }
    }
}

impl<M, SPI, PINS, CS, D> ErrorType for SpiSharedDevice<'_, M, SPI, PINS, CS, D>
where
    M: RawMutex,
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
    CS: PinCS<SPI>,
    D: DelayNs,
{
    type Error = ErrorKind;
}

impl<M, SPI, PINS, CS, D> SpiDevice for SpiSharedDevice<'_, M, SPI, PINS, CS, D>
where
    M: RawMutex,
    SPI: SpiX,
    PINS: PinsNoCS<SPI> + PinsFull<SPI>,
    CS: PinCS<SPI>,
    D: DelayNs,
{
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        let mut bus = self.bus.lock().await;
        unsafe { bus.configure(&self.config, Some(CS::CS_INDEX)) };
        bus.start_frame();

        let mut res = Ok(());
        for operation in operations.iter_mut() {
            res = match operation {
                Operation::Read(read) => bus.read(read).await,
                Operation::Write(write) => bus.write(write).await,
                Operation::Transfer(read, write) => bus.transfer(read, write).await,
                Operation::TransferInPlace(read_write) => bus.transfer_in_place(read_write).await,
                Operation::DelayNs(ns) => {
                    self.delay.delay_ns(*ns).await;
                    Ok(())
                }
            };
            if res.is_err() {
                break;
            }
        }

        if res.is_ok() {
            bus.flush().await?;
        }
        bus.end_frame();

        Ok(())
    }
}
