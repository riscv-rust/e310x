//! E31 core peripherals

pub mod counters;

use e310x::{Clint, Plic};

/// Core peripherals
pub struct CorePeripherals {
    /// Core Local Interruptor (CLINT)
    pub clint: Clint,
    /// Platform-Level Interrupt Controller (PLIC)
    pub plic: Plic,
    /// Performance counters
    pub counters: counters::PerformanceCounters,
}

impl CorePeripherals {
    pub(crate) const fn new(clint: Clint, plic: Plic) -> Self {
        Self {
            clint,
            plic,
            counters: counters::PerformanceCounters::new(),
        }
    }

    /// Steal the peripherals
    ///
    /// # Safety
    ///
    /// Using this function may break the guarantees of the singleton pattern.
    pub unsafe fn steal() -> Self {
        Self::new(Clint::steal(), Plic::steal())
    }
}
