//! E31 core peripherals

pub mod clint;
pub mod counters;
pub mod plic;

/// Core peripherals
pub struct CorePeripherals {
    /// Core-Local Interruptor
    pub clint: clint::Clint,

    /// Platform-Level Interrupt Controller
    pub plic: plic::Plic,

    /// Performance counters
    pub counters: counters::PerformanceCounters,
}

impl CorePeripherals {
    pub(crate) fn new(clint: e310x::Clint, plic: e310x::Plic) -> Self {
        Self {
            clint: clint.into(),
            plic: plic.into(),
            counters: counters::PerformanceCounters::new(),
        }
    }

    /// Steal the peripherals
    ///
    /// # Safety
    ///
    /// Using this function may break the guarantees of the singleton pattern.
    pub unsafe fn steal() -> Self {
        let p = e310x::Peripherals::steal();
        Self::new(p.clint, p.plic)
    }
}
