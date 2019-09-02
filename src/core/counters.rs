//! Performance counters

use riscv::register::{mcycle, minstret, mhpmcounter3, mhpmcounter4};

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

/// Opaque mhpmcounter3 register.
pub struct MHPMCOUNTER3;

impl MHPMCOUNTER3 {
    /// Read mhpmcounter3 and mhpmcounter3h registers.
    #[inline]
    pub fn value(&self) -> u64 {
        mhpmcounter3::read64()
    }
}

/// Opaque mhpmcounter4 register.
pub struct MHPMCOUNTER4;

impl MHPMCOUNTER4 {
    /// Read mhpmcounter4 and mhpmcounter4h registers.
    #[inline]
    pub fn value(&self) -> u64 {
        mhpmcounter4::read64()
    }
}

/// Performance counters
pub struct PerformanceCounters {
    /// 64-bit mcycle counter
    pub mcycle: MCYCLE,
    /// 64-bit minstret counter
    pub minstret: MINSTRET,
    /// 40-bit mhpmcounter3 counter
    pub mhpmcounter3: MHPMCOUNTER3,
    /// 40-bit mhpmcounter4 counter
    pub mhpmcounter4: MHPMCOUNTER4,

    // TODO: mhpmevent3, mhpmevent4
}

impl PerformanceCounters {
    pub(crate) fn new() -> Self {
        Self {
            mcycle: MCYCLE,
            minstret: MINSTRET,
            mhpmcounter3: MHPMCOUNTER3,
            mhpmcounter4: MHPMCOUNTER4
        }
    }
}
