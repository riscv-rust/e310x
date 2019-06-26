#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMUCAUSE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `resetcause`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETCAUSER {
    #[doc = "Power-on reset"]
    POWERON,
    #[doc = "External reset"]
    EXTERNAL,
    #[doc = "Watchdog reset"]
    WATCHDOG,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESETCAUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESETCAUSER::POWERON => 0,
            RESETCAUSER::EXTERNAL => 1,
            RESETCAUSER::WATCHDOG => 2,
            RESETCAUSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESETCAUSER {
        match value {
            0 => RESETCAUSER::POWERON,
            1 => RESETCAUSER::EXTERNAL,
            2 => RESETCAUSER::WATCHDOG,
            i => RESETCAUSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWERON`"]
    #[inline]
    pub fn is_power_on(&self) -> bool {
        *self == RESETCAUSER::POWERON
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == RESETCAUSER::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `WATCHDOG`"]
    #[inline]
    pub fn is_watchdog(&self) -> bool {
        *self == RESETCAUSER::WATCHDOG
    }
}
#[doc = "Possible values of the field `wakeupcause`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPCAUSER {
    #[doc = "Reset wakeup"]
    RESET,
    #[doc = "RTC wakeup"]
    RTC,
    #[doc = "Digital input wakeup"]
    DIGITAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAKEUPCAUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAKEUPCAUSER::RESET => 0,
            WAKEUPCAUSER::RTC => 1,
            WAKEUPCAUSER::DIGITAL => 2,
            WAKEUPCAUSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAKEUPCAUSER {
        match value {
            0 => WAKEUPCAUSER::RESET,
            1 => WAKEUPCAUSER::RTC,
            2 => WAKEUPCAUSER::DIGITAL,
            i => WAKEUPCAUSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WAKEUPCAUSER::RESET
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == WAKEUPCAUSER::RTC
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline]
    pub fn is_digital(&self) -> bool {
        *self == WAKEUPCAUSER::DIGITAL
    }
}
#[doc = "Values that can be written to the field `resetcause`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETCAUSEW {
    #[doc = "Power-on reset"]
    POWERON,
    #[doc = "External reset"]
    EXTERNAL,
    #[doc = "Watchdog reset"]
    WATCHDOG,
}
impl RESETCAUSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESETCAUSEW::POWERON => 0,
            RESETCAUSEW::EXTERNAL => 1,
            RESETCAUSEW::WATCHDOG => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETCAUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETCAUSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETCAUSEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power-on reset"]
    #[inline]
    pub fn power_on(self) -> &'a mut W {
        self.variant(RESETCAUSEW::POWERON)
    }
    #[doc = "External reset"]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(RESETCAUSEW::EXTERNAL)
    }
    #[doc = "Watchdog reset"]
    #[inline]
    pub fn watchdog(self) -> &'a mut W {
        self.variant(RESETCAUSEW::WATCHDOG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `wakeupcause`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPCAUSEW {
    #[doc = "Reset wakeup"]
    RESET,
    #[doc = "RTC wakeup"]
    RTC,
    #[doc = "Digital input wakeup"]
    DIGITAL,
}
impl WAKEUPCAUSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAKEUPCAUSEW::RESET => 0,
            WAKEUPCAUSEW::RTC => 1,
            WAKEUPCAUSEW::DIGITAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPCAUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPCAUSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPCAUSEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset wakeup"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(WAKEUPCAUSEW::RESET)
    }
    #[doc = "RTC wakeup"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(WAKEUPCAUSEW::RTC)
    }
    #[doc = "Digital input wakeup"]
    #[inline]
    pub fn digital(self) -> &'a mut W {
        self.variant(WAKEUPCAUSEW::DIGITAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn resetcause(&self) -> RESETCAUSER {
        RESETCAUSER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn wakeupcause(&self) -> WAKEUPCAUSER {
        WAKEUPCAUSER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn resetcause(&mut self) -> _RESETCAUSEW {
        _RESETCAUSEW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn wakeupcause(&mut self) -> _WAKEUPCAUSEW {
        _WAKEUPCAUSEW { w: self }
    }
}
