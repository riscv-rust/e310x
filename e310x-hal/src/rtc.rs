//! RTC
#![allow(missing_docs)]

use e310x::Rtc as RTC;

pub trait RtcExt {
    fn constrain(self) -> Rtc;
}

impl RtcExt for RTC {
    fn constrain(self) -> Rtc {
        Rtc { _0: () }
    }
}

pub struct Rtc {
    _0: (),
}

impl Rtc {
    #[inline]
    pub fn is_pending(&self) -> bool {
        unsafe { RTC::steal() }.rtccfg().read().cmpip().bit()
    }

    #[inline]
    pub fn set_scale(&mut self, scale: u8) {
        unsafe { RTC::steal().rtccfg().modify(|_, w| w.scale().bits(scale)) };
    }

    #[inline]
    pub fn enable(&mut self) {
        unsafe { RTC::steal() }
            .rtccfg()
            .modify(|_, w| w.enalways().bit(true));
    }

    #[inline]
    pub fn disable(&mut self) {
        unsafe { RTC::steal() }
            .rtccfg()
            .modify(|_, w| w.enalways().bit(false));
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        unsafe { RTC::steal() }.rtccfg().read().enalways().bit()
    }

    #[inline]
    pub fn rtc_lo(&self) -> u32 {
        unsafe { RTC::steal() }.rtclo().read().bits()
    }

    #[inline]
    pub fn rtc_hi(&self) -> u32 {
        unsafe { RTC::steal() }.rtchi().read().bits()
    }

    pub fn rtc(&self) -> u64 {
        loop {
            let hi = self.rtc_hi();
            let lo = self.rtc_lo();
            if hi == self.rtc_hi() {
                return ((hi as u64) << 32) | lo as u64;
            }
        }
    }

    #[inline]
    pub fn set_rtc_lo(&mut self, value: u32) {
        unsafe { RTC::steal().rtclo().write(|w| w.bits(value)) };
    }

    #[inline]
    pub fn set_rtc_hi(&mut self, value: u16) {
        unsafe { RTC::steal().rtchi().write(|w| w.value().bits(value)) };
    }

    pub fn set_rtc(&mut self, value: u64) {
        self.set_rtc_hi((value >> 32) as u16);
        self.set_rtc_lo(value as u32);
    }

    #[inline]
    pub fn rtccmp(&self) -> u32 {
        unsafe { RTC::steal() }.rtccmp().read().bits()
    }

    #[inline]
    pub fn set_rtccmp(&mut self, value: u32) {
        unsafe { RTC::steal().rtccmp().write(|w| w.bits(value)) };
    }
}
