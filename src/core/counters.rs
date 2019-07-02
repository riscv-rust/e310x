//! Performance counters

use riscv::register::{mcycle, minstret};

/// Opaque mcycle register
pub struct MCYCLE;

impl MCYCLE {
    /// Read mcycle and mcycleh registers.
    #[inline]
    pub fn value(&self) -> u64 {
        mcycle::read64()
    }
}

/// Opaque minstret register.
pub struct MINSTRET;

impl MINSTRET {
    /// Read minstret and minstreth registers.
    #[inline]
    pub fn value(&self) -> u64 {
        minstret::read64()
    }
}


/// Performance counters
pub struct PerformanceCounters {
    /// 64-bit mcycle counter
    pub mcycle: MCYCLE,
    /// 64-bit minstret counter
    pub minstret: MINSTRET,

    // TODO: mhpmcounter3, mhpmcounter4
    // TODO: mhpmevent3, mhpmevent4
}

impl PerformanceCounters {
    pub(crate) fn new() -> Self {
        Self {
            mcycle: MCYCLE,
            minstret: MINSTRET,
        }
    }
}
