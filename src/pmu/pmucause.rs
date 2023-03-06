#[doc = "Register `pmucause` reader"]
pub struct R(crate::R<PMUCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pmucause` writer"]
pub struct W(crate::W<PMUCAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMUCAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUCAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wakeupcause` reader - "]
pub type WAKEUPCAUSE_R = crate::FieldReader<u8, WAKEUPCAUSE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WAKEUPCAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAKEUPCAUSE_A> {
        match self.bits {
            0 => Some(WAKEUPCAUSE_A::RESET),
            1 => Some(WAKEUPCAUSE_A::RTC),
            2 => Some(WAKEUPCAUSE_A::DIGITAL),
            _ => None,
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
#[doc = "Field `wakeupcause` writer - "]
pub type WAKEUPCAUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PMUCAUSE_SPEC, u8, WAKEUPCAUSE_A, 2, O>;
impl<'a, const O: u8> WAKEUPCAUSE_W<'a, O> {
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
}
#[doc = "Field `resetcause` reader - "]
pub type RESETCAUSE_R = crate::FieldReader<u8, RESETCAUSE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESETCAUSE_A {
    #[doc = "0: Power-on reset"]
    POWER_ON = 0,
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
impl RESETCAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESETCAUSE_A> {
        match self.bits {
            0 => Some(RESETCAUSE_A::POWER_ON),
            1 => Some(RESETCAUSE_A::EXTERNAL),
            2 => Some(RESETCAUSE_A::WATCHDOG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_ON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == RESETCAUSE_A::POWER_ON
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
#[doc = "Field `resetcause` writer - "]
pub type RESETCAUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PMUCAUSE_SPEC, u8, RESETCAUSE_A, 2, O>;
impl<'a, const O: u8> RESETCAUSE_W<'a, O> {
    #[doc = "Power-on reset"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(RESETCAUSE_A::POWER_ON)
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
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wakeupcause(&self) -> WAKEUPCAUSE_R {
        WAKEUPCAUSE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn resetcause(&self) -> RESETCAUSE_R {
        RESETCAUSE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wakeupcause(&mut self) -> WAKEUPCAUSE_W<0> {
        WAKEUPCAUSE_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn resetcause(&mut self) -> RESETCAUSE_W<8> {
        RESETCAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmucause](index.html) module"]
pub struct PMUCAUSE_SPEC;
impl crate::RegisterSpec for PMUCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmucause::R](R) reader structure"]
impl crate::Readable for PMUCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmucause::W](W) writer structure"]
impl crate::Writable for PMUCAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pmucause to value 0"]
impl crate::Resettable for PMUCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
