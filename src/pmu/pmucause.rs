#[doc = "Register `pmucause` reader"]
pub type R = crate::R<PMUCAUSE_SPEC>;
#[doc = "Register `pmucause` writer"]
pub type W = crate::W<PMUCAUSE_SPEC>;
#[doc = "Field `wakeupcause` reader - "]
pub type WAKEUPCAUSE_R = crate::FieldReader<WAKEUPCAUSE_A>;
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
impl crate::FieldSpec for WAKEUPCAUSE_A {
    type Ux = u8;
}
impl WAKEUPCAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAKEUPCAUSE_A> {
        match self.bits {
            0 => Some(WAKEUPCAUSE_A::RESET),
            1 => Some(WAKEUPCAUSE_A::RTC),
            2 => Some(WAKEUPCAUSE_A::DIGITAL),
            _ => None,
        }
    }
    #[doc = "Reset wakeup"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WAKEUPCAUSE_A::RESET
    }
    #[doc = "RTC wakeup"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == WAKEUPCAUSE_A::RTC
    }
    #[doc = "Digital input wakeup"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == WAKEUPCAUSE_A::DIGITAL
    }
}
#[doc = "Field `wakeupcause` writer - "]
pub type WAKEUPCAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAKEUPCAUSE_A>;
impl<'a, REG> WAKEUPCAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset wakeup"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUPCAUSE_A::RESET)
    }
    #[doc = "RTC wakeup"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUPCAUSE_A::RTC)
    }
    #[doc = "Digital input wakeup"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUPCAUSE_A::DIGITAL)
    }
}
#[doc = "Field `resetcause` reader - "]
pub type RESETCAUSE_R = crate::FieldReader<RESETCAUSE_A>;
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
impl crate::FieldSpec for RESETCAUSE_A {
    type Ux = u8;
}
impl RESETCAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESETCAUSE_A> {
        match self.bits {
            0 => Some(RESETCAUSE_A::POWER_ON),
            1 => Some(RESETCAUSE_A::EXTERNAL),
            2 => Some(RESETCAUSE_A::WATCHDOG),
            _ => None,
        }
    }
    #[doc = "Power-on reset"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == RESETCAUSE_A::POWER_ON
    }
    #[doc = "External reset"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == RESETCAUSE_A::EXTERNAL
    }
    #[doc = "Watchdog reset"]
    #[inline(always)]
    pub fn is_watchdog(&self) -> bool {
        *self == RESETCAUSE_A::WATCHDOG
    }
}
#[doc = "Field `resetcause` writer - "]
pub type RESETCAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RESETCAUSE_A>;
impl<'a, REG> RESETCAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power-on reset"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut crate::W<REG> {
        self.variant(RESETCAUSE_A::POWER_ON)
    }
    #[doc = "External reset"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(RESETCAUSE_A::EXTERNAL)
    }
    #[doc = "Watchdog reset"]
    #[inline(always)]
    pub fn watchdog(self) -> &'a mut crate::W<REG> {
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
    #[must_use]
    pub fn wakeupcause(&mut self) -> WAKEUPCAUSE_W<PMUCAUSE_SPEC> {
        WAKEUPCAUSE_W::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn resetcause(&mut self) -> RESETCAUSE_W<PMUCAUSE_SPEC> {
        RESETCAUSE_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PMU Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUCAUSE_SPEC;
impl crate::RegisterSpec for PMUCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucause::R`](R) reader structure"]
impl crate::Readable for PMUCAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmucause::W`](W) writer structure"]
impl crate::Writable for PMUCAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pmucause to value 0"]
impl crate::Resettable for PMUCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
