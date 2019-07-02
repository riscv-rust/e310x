//! E31 core peripherals

pub mod clint;
pub mod counters;

/// Core peripherals
pub struct CorePeripherals {
    /// Core-Local Interruptor
    pub clint: clint::Clint,

    /// Performance counters
    pub counters: counters::PerformanceCounters,
}
