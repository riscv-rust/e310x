use crate::spi::{Pins, PinsFull, SpiBus as Bus, SpiConfig, SpiX};
use embedded_hal_async::{
    delay::DelayNs,
    spi::{self, ErrorType, Operation, SpiBus, SpiDevice},
};

/// SPI exclusive device abstraction with async delay support.
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
    /// Create [`SpiExclusiveDevice`] using existing [`SpiBus`](Bus) with the given [`SpiConfig`]
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
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        self.bus.start_frame();

        let mut res = Ok(());
        for operation in operations.iter_mut() {
            res = match operation {
                Operation::Read(read) => self.bus.read(read).await,
                Operation::Write(write) => self.bus.write(write).await,
                Operation::Transfer(read, write) => self.bus.transfer(read, write).await,
                Operation::TransferInPlace(read_write) => {
                    self.bus.transfer_in_place(read_write).await
                }
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
            self.bus.flush().await?;
        }
        self.bus.end_frame();
        res
    }
}
