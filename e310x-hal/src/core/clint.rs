//! Core-Local Interruptor

use e310x::Clint as CLINT;

macro_rules! read64 {
    ($hi:expr, $lo:expr) => {
        loop {
            let hi = $hi;
            let lo = $lo;
            if hi == $hi {
                return ((hi as u64) << 32) | lo as u64;
            }
        }
    };
}

/// Opaque msip register
pub struct MSIP {
    _0: (),
}

impl MSIP {
    /// Set msip register value
    pub fn set_value(&mut self, value: bool) {
        unsafe {
            CLINT::steal()
                .msip()
                .write(|w| if value { w.bits(1) } else { w.bits(0) })
        }
    }
}

/// Opaque mtime register
pub struct MTIME;

impl MTIME {
    /// Read mtime register.
    #[inline]
    pub fn mtime_lo(&self) -> u32 {
        unsafe { CLINT::steal() }.mtime().read().bits()
    }

    /// Read mtimeh register.
    #[inline]
    pub fn mtime_hi(&self) -> u32 {
        unsafe { CLINT::steal() }.mtimeh().read().bits()
    }

    /// Read mtime and mtimeh registers.
    pub fn mtime(&self) -> u64 {
        read64!(self.mtime_hi(), self.mtime_lo())
    }
}

/// Opaque mtimecmp register
pub struct MTIMECMP {
    _0: (),
}

impl MTIMECMP {
    /// Read mtimecmp register.
    #[inline]
    pub fn mtimecmp_lo(&self) -> u32 {
        unsafe { CLINT::steal() }.mtimecmp().read().bits()
    }

    /// Read mtimecmph register.
    #[inline]
    pub fn mtimecmp_hi(&self) -> u32 {
        unsafe { CLINT::steal() }.mtimecmph().read().bits()
    }

    /// Read mtimecmp and mtimecmph registers.
    pub fn mtimecmp(&self) -> u64 {
        read64!(self.mtimecmp_hi(), self.mtimecmp_lo())
    }

    /// Write mtimecmp register
    #[inline]
    pub fn set_mtimecmp_lo(&mut self, value: u32) {
        unsafe { CLINT::steal().mtimecmp().write(|w| w.bits(value)) };
    }

    /// Write mtimecmph register
    #[inline]
    pub fn set_mtimecmp_hi(&mut self, value: u32) {
        unsafe { CLINT::steal().mtimecmph().write(|w| w.bits(value)) };
    }

    /// Write mtimecmp and mtimecmph registers.
    pub fn set_mtimecmp(&mut self, value: u64) {
        // Volume II: RISC-V Privileged Architectures V1.10 p.31, figure 3.15
        self.set_mtimecmp_lo(0xffff_ffff); // No smaller than old value
        self.set_mtimecmp_hi((value >> 32) as u32); // New value
        self.set_mtimecmp_lo(value as u32); // New value
    }
}

/// Core-Local Interruptor abstraction
pub struct Clint {
    /// Opaque msip register
    pub msip: MSIP,
    /// Opaque mtimecmp register
    pub mtimecmp: MTIMECMP,
    /// Opaque mtime register
    pub mtime: MTIME,
}

impl From<CLINT> for Clint {
    fn from(_: CLINT) -> Self {
        Clint {
            msip: MSIP { _0: () },
            mtimecmp: MTIMECMP { _0: () },
            mtime: MTIME,
        }
    }
}
