#[doc = "Reader of register pmucause"]
pub type R = crate::R<u32, super::PMUCAUSE>;
#[doc = "Writer for register pmucause"]
pub type W = crate::W<u32, super::PMUCAUSE>;
#[doc = "Register pmucause `reset()`'s with value 0"]
impl crate::ResetValue for super::PMUCAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESETCAUSE_A {
    #[doc = "0: Power-on reset"]
    POWERON = 0,
    #[doc = "1: External reset"]
    EXTERNAL = 1,
    #[doc = "2: Watchdog reset"]
    WATCHDOG = 2,
}
impl From<RESETCAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: RESETCAUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `resetcause`"]
pub type RESETCAUSE_R = crate::R<u8, RESETCAUSE_A>;
impl RESETCAUSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RESETCAUSE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RESETCAUSE_A::POWERON),
            1 => Val(RESETCAUSE_A::EXTERNAL),
            2 => Val(RESETCAUSE_A::WATCHDOG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWERON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == RESETCAUSE_A::POWERON
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == RESETCAUSE_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `WATCHDOG`"]
    #[inline(always)]
    pub fn is_watchdog(&self) -> bool {
        *self == RESETCAUSE_A::WATCHDOG
    }
}
#[doc = "Write proxy for field `resetcause`"]
pub struct RESETCAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETCAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETCAUSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Power-on reset"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(RESETCAUSE_A::POWERON)
    }
    #[doc = "External reset"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(RESETCAUSE_A::EXTERNAL)
    }
    #[doc = "Watchdog reset"]
    #[inline(always)]
    pub fn watchdog(self) -> &'a mut W {
        self.variant(RESETCAUSE_A::WATCHDOG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAKEUPCAUSE_A {
    #[doc = "0: Reset wakeup"]
    RESET = 0,
    #[doc = "1: RTC wakeup"]
    RTC = 1,
    #[doc = "2: Digital input wakeup"]
    DIGITAL = 2,
}
impl From<WAKEUPCAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: WAKEUPCAUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `wakeupcause`"]
pub type WAKEUPCAUSE_R = crate::R<u8, WAKEUPCAUSE_A>;
impl WAKEUPCAUSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAKEUPCAUSE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAKEUPCAUSE_A::RESET),
            1 => Val(WAKEUPCAUSE_A::RTC),
            2 => Val(WAKEUPCAUSE_A::DIGITAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WAKEUPCAUSE_A::RESET
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == WAKEUPCAUSE_A::RTC
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == WAKEUPCAUSE_A::DIGITAL
    }
}
#[doc = "Write proxy for field `wakeupcause`"]
pub struct WAKEUPCAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPCAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPCAUSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset wakeup"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WAKEUPCAUSE_A::RESET)
    }
    #[doc = "RTC wakeup"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(WAKEUPCAUSE_A::RTC)
    }
    #[doc = "Digital input wakeup"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(WAKEUPCAUSE_A::DIGITAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn resetcause(&self) -> RESETCAUSE_R {
        RESETCAUSE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wakeupcause(&self) -> WAKEUPCAUSE_R {
        WAKEUPCAUSE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn resetcause(&mut self) -> RESETCAUSE_W {
        RESETCAUSE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wakeupcause(&mut self) -> WAKEUPCAUSE_W {
        WAKEUPCAUSE_W { w: self }
    }
}
