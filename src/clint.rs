//! Clint

use riscv::register::{mcycle, mcycleh, minstret, minstreth, mie, mip};
use e310x::CLINT;

macro_rules! read64 {
    ($hi:expr, $lo:expr) => {
        loop {
            let hi = $hi;
            let lo = $lo;
            if hi == $hi {
                return ((hi as u64) << 32) | lo as u64;
            }
        }
    }
}

/// ClintExt trait extends the CLINT peripheral.
pub trait ClintExt {
    /// The parts to split the GPIO into.
    type Parts;

    /// Splits the GPIO block into independent pins and registers.
    fn split(self) -> Self::Parts;
}

/// Opaque MTIME register
pub struct MTIME;

impl MTIME {
    /// Read mtime register.
    #[inline]
    pub fn mtime_lo(&self) -> u32 {
        unsafe { (*CLINT::ptr()).mtime.read().bits() }
    }

    /// Read mtimeh register.
    #[inline]
    pub fn mtime_hi(&self) -> u32 {
        unsafe { (*CLINT::ptr()).mtimeh.read().bits() }
    }

    /// Read mtime and mtimeh registers.
    pub fn mtime(&self) -> u64 {
        read64!(self.mtime_hi(), self.mtime_lo())
    }
}

/// Opaque MTIMECMP register
pub struct MTIMECMP {
    _0: (),
}

impl MTIMECMP {
    /// Read mtimecmp register.
    #[inline]
    pub fn mtimecmp_lo(&self) -> u32 {
        unsafe { (*CLINT::ptr()).mtimecmp.read().bits() }
    }

    /// Read mtimecmph register.
    #[inline]
    pub fn mtimecmp_hi(&self) -> u32 {
        unsafe { (*CLINT::ptr()).mtimecmph.read().bits() }
    }

    /// Read mtimecmp and mtimecmph registers.
    pub fn mtimecmp(&self) -> u64 {
        read64!(self.mtimecmp_hi(), self.mtimecmp_lo())
    }

    /// Write mtimecmp register
    #[inline]
    pub fn set_mtimecmp_lo(&mut self, value: u32) {
        unsafe { (*CLINT::ptr()).mtimecmp.write(|w| w.bits(value)) };
    }

    /// Write mtimecmph register
    #[inline]
    pub fn set_mtimecmp_hi(&mut self, value: u32) {
        unsafe { (*CLINT::ptr()).mtimecmph.write(|w| w.bits(value)) };
    }

    /// Write mtimecmp and mtimecmph registers.
    pub fn set_mtimecmp(&mut self, value: u64) {
        self.set_mtimecmp_hi((value >> 32) as u32);
        self.set_mtimecmp_lo(value as u32);
    }
}

/// Opaque mcycle register
pub struct MCYCLE {
    _0: (),
}

impl MCYCLE {
    /// Read mcycle register.
    #[inline]
    pub fn mcycle_lo(&self) -> u32 {
        mcycle::read() as u32
    }

    /// Read mcycleh register.
    #[inline]
    pub fn mcycle_hi(&self) -> u32 {
        mcycleh::read() as u32
    }

    /// Read mcycle and mcycleh registers.
    pub fn mcycle(&self) -> u64 {
        read64!(mcycleh::read(), mcycle::read())
    }
}

/// Opaque minstret register.
pub struct MINSTRET {
    _0: (),
}

impl MINSTRET {
    /// Read minstret register.
    #[inline]
    pub fn minstret_lo(&self) -> u32 {
        minstret::read() as u32
    }

    /// Read minstreth register.
    #[inline]
    pub fn minstret_hi(&self) -> u32 {
        minstreth::read() as u32
    }

    /// Read minstret and minstreth registers.
    pub fn minstret(&self) -> u64 {
        read64!(self.minstret_hi(), self.minstret_lo())
    }
}

/// Opaque mtimer interrupt handling.
pub struct MTIMER {
    _0: (),
}

impl MTIMER {
    /// Enable Machine-Timer interrupt.
    pub fn enable(&mut self) {
        unsafe { mie::set_mtimer() };
    }

    /// Disable Machine-Timer interrupt.
    pub fn disable(&mut self) {
        unsafe { mie::clear_mtimer(); }
    }

    /// Check if the Machine-Timer is interrupt pending.
    pub fn is_pending(&self) -> bool {
        mip::read().mtimer()
    }
}

/// Parts of CLINT peripheral for fine grained permission control.
pub struct ClintParts {
    /// Opaque mtimecmp register
    pub mtimecmp: MTIMECMP,
    /// Opaque mtime register
    pub mtime: MTIME,
    /// Opaque mcycle register
    pub mcycle: MCYCLE,
    /// Opaque minstret register
    pub minstret: MINSTRET,
    /// Opaque mtimer register
    pub mtimer: MTIMER,
}

impl ClintExt for CLINT {
    type Parts = ClintParts;

    fn split(self) -> ClintParts {
        ClintParts {
            mtimecmp: MTIMECMP { _0: () },
            mtime: MTIME,
            mcycle: MCYCLE { _0: () },
            minstret: MINSTRET { _0: () },
            mtimer: MTIMER { _0: () },
        }
    }
}
