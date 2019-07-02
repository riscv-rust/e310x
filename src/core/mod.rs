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
    pub(crate) fn new(clint: e310x::CLINT, plic: e310x::PLIC) -> Self {
        Self {
            clint: clint.into(),
            plic: plic.into(),
            counters: counters::PerformanceCounters::new()
        }
    }

    /// Steal the peripherals
    pub unsafe fn steal() -> Self {
        let p = e310x::Peripherals::steal();
        Self::new(p.CLINT, p.PLIC)
    }
}
