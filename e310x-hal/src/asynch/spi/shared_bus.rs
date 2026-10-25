use super::SpiSharedDevice;
use crate::spi::{PinCS, PinsNoCS, SpiBus, SpiConfig, SpiX};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
use embedded_hal_async::delay::DelayNs;

/// Used to hold the [SpiBus] instance so it can be used for multiple [SpiSharedDevice] instances.
pub struct SharedBus<M: RawMutex, SPI, PINS>(Mutex<M, SpiBus<SPI, PINS>>);

impl<M: RawMutex, SPI, PINS> SharedBus<M, SPI, PINS>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
{
    pub(crate) fn new(bus: SpiBus<SPI, PINS>) -> Self {
        Self(Mutex::new(bus))
    }

    /// Create a new [`SpiSharedDevice`] for use on this bus
    pub fn new_device<'a, CS, D: DelayNs>(
        &'a self,
        cs: CS,
        config: &SpiConfig,
        delay: D,
    ) -> SpiSharedDevice<'a, M, SPI, PINS, CS, D>
    where
        CS: PinCS<SPI>,
        PINS: PinsNoCS<SPI>,
    {
        SpiSharedDevice::new(self, cs, config, delay)
    }

    /// Lock the Mutex to access the underlying SpiBus
    pub async fn lock(&self) -> MutexGuard<'_, M, SpiBus<SPI, PINS>> {
        self.0.lock().await
    }
}
