//! Watchdog
#![allow(missing_docs)]
use e310x::WDOG;

pub trait WdogExt {
    fn configure(self) -> WdogCfg;
}

impl WdogExt for WDOG {
    fn configure(self) -> WdogCfg {
        WdogCfg {
            _0: (),
            enable: false,
            awake: false,
            reset: false,
            zero_cmp: false,
            scale: 0,
        }
    }
}

pub struct WdogCfg {
    _0: (),
    enable: bool,
    awake: bool,
    reset: bool,
    zero_cmp: bool,
    scale: u8,
}

impl WdogCfg {
    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    pub fn enable_awake(mut self) -> Self {
        self.awake = true;
        self
    }

    pub fn enable_reset(mut self) -> Self {
        self.reset = true;
        self
    }

    pub fn enable_zero_cmp(mut self) -> Self {
        self.zero_cmp = true;
        self
    }

    pub fn scale(mut self, scale: u8) -> Self {
        self.scale = scale;
        self
    }

    pub fn freeze(self) -> Wdog {
        unsafe {
            (*WDOG::ptr()).wdogkey.write(|w| w.bits(0x51F15E));
            (*WDOG::ptr()).wdogcfg.write(|w| w
                                         .scale().bits(self.scale)
                                         .rsten().bit(self.reset)
                                         .zerocmp().bit(self.zero_cmp)
                                         .enalways().bit(self.enable)
                                         .encoreawake().bit(self.awake));
        }
        Wdog { _0: () }
    }
}

pub struct Wdog {
    _0: (),
}

impl Wdog {
    #[inline]
    fn unlock(&mut self) {
        unsafe { (*WDOG::ptr()).wdogkey.write(|w| w.bits(0x51F15E)) };
    }

    pub fn is_pending(&self) -> bool {
        unsafe { (*WDOG::ptr()).wdogcfg.read().cmpip().bit() }
    }

    pub fn feed(&mut self) {
        self.unlock();
        unsafe { (*WDOG::ptr()).wdogfeed.write(|w| w.bits(0xD09F00D)) };
    }

    pub fn cmp(&self) -> u16 {
        unsafe { (*WDOG::ptr()).wdogcmp.read().value().bits() }
    }

    pub fn set_cmp(&mut self, value: u16) {
        unsafe { (*WDOG::ptr()).wdogcmp.write(|w| w.value().bits(value)) };
    }
}
