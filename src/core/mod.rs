//! E31 core peripherals

pub mod clint;
pub mod counters;
pub mod plic;

/// Core peripherals
pub struct CorePeripherals {
    /// Core-Local Interruptor
    pub clint: clint::Clint,

    /// Platform-Level Interrupt Controller
    pub plic: plic::PlicParts,

    /// Performance counters
    pub counters: counters::PerformanceCounters,
}
