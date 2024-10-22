//! E31 core peripherals

pub mod counters;

pub use e310x::{CLINT, PLIC};

/// Core peripherals
pub struct CorePeripherals {
    /// Performance counters
    pub counters: counters::PerformanceCounters,
}

impl CorePeripherals {
    pub(crate) fn new() -> Self {
        Self {
            counters: counters::PerformanceCounters::new(),
        }
    }

    /// Steal the peripherals
    ///
    /// # Safety
    ///
    /// Using this function may break the guarantees of the singleton pattern.
    pub unsafe fn steal() -> Self {
        Self::new()
    }
}
