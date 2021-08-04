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
#[doc = "Field `resetcause` reader - "]
pub struct RESETCAUSE_R(crate::FieldReader<u8, RESETCAUSE_A>);
impl RESETCAUSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESETCAUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESETCAUSE_A> {
        match self.bits {
            0 => Some(RESETCAUSE_A::POWERON),
            1 => Some(RESETCAUSE_A::EXTERNAL),
            2 => Some(RESETCAUSE_A::WATCHDOG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWERON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        **self == RESETCAUSE_A::POWERON
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == RESETCAUSE_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `WATCHDOG`"]
    #[inline(always)]
    pub fn is_watchdog(&self) -> bool {
        **self == RESETCAUSE_A::WATCHDOG
    }
}
impl core::ops::Deref for RESETCAUSE_R {
    type Target = crate::FieldReader<u8, RESETCAUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `resetcause` writer - "]
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
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
#[doc = "Field `wakeupcause` reader - "]
pub struct WAKEUPCAUSE_R(crate::FieldReader<u8, WAKEUPCAUSE_A>);
impl WAKEUPCAUSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAKEUPCAUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == WAKEUPCAUSE_A::RESET
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        **self == WAKEUPCAUSE_A::RTC
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        **self == WAKEUPCAUSE_A::DIGITAL
    }
}
impl core::ops::Deref for WAKEUPCAUSE_R {
    type Target = crate::FieldReader<u8, WAKEUPCAUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wakeupcause` writer - "]
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
