#[doc = "Register `pmucause` reader"]
pub type R = crate::R<PmucauseSpec>;
#[doc = "Register `pmucause` writer"]
pub type W = crate::W<PmucauseSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wakeupcause {
    #[doc = "0: Reset wakeup"]
    Reset = 0,
    #[doc = "1: RTC wakeup"]
    Rtc = 1,
    #[doc = "2: Digital input wakeup"]
    Digital = 2,
}
impl From<Wakeupcause> for u8 {
    #[inline(always)]
    fn from(variant: Wakeupcause) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wakeupcause {
    type Ux = u8;
}
impl crate::IsEnum for Wakeupcause {}
#[doc = "Field `wakeupcause` reader - "]
pub type WakeupcauseR = crate::FieldReader<Wakeupcause>;
impl WakeupcauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wakeupcause> {
        match self.bits {
            0 => Some(Wakeupcause::Reset),
            1 => Some(Wakeupcause::Rtc),
            2 => Some(Wakeupcause::Digital),
            _ => None,
        }
    }
    #[doc = "Reset wakeup"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Wakeupcause::Reset
    }
    #[doc = "RTC wakeup"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Wakeupcause::Rtc
    }
    #[doc = "Digital input wakeup"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == Wakeupcause::Digital
    }
}
#[doc = "Field `wakeupcause` writer - "]
pub type WakeupcauseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wakeupcause>;
impl<'a, REG> WakeupcauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset wakeup"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeupcause::Reset)
    }
    #[doc = "RTC wakeup"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeupcause::Rtc)
    }
    #[doc = "Digital input wakeup"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeupcause::Digital)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resetcause {
    #[doc = "0: Power-on reset"]
    PowerOn = 0,
    #[doc = "1: External reset"]
    External = 1,
    #[doc = "2: Watchdog reset"]
    Watchdog = 2,
}
impl From<Resetcause> for u8 {
    #[inline(always)]
    fn from(variant: Resetcause) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resetcause {
    type Ux = u8;
}
impl crate::IsEnum for Resetcause {}
#[doc = "Field `resetcause` reader - "]
pub type ResetcauseR = crate::FieldReader<Resetcause>;
impl ResetcauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Resetcause> {
        match self.bits {
            0 => Some(Resetcause::PowerOn),
            1 => Some(Resetcause::External),
            2 => Some(Resetcause::Watchdog),
            _ => None,
        }
    }
    #[doc = "Power-on reset"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == Resetcause::PowerOn
    }
    #[doc = "External reset"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Resetcause::External
    }
    #[doc = "Watchdog reset"]
    #[inline(always)]
    pub fn is_watchdog(&self) -> bool {
        *self == Resetcause::Watchdog
    }
}
#[doc = "Field `resetcause` writer - "]
pub type ResetcauseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resetcause>;
impl<'a, REG> ResetcauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power-on reset"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut crate::W<REG> {
        self.variant(Resetcause::PowerOn)
    }
    #[doc = "External reset"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Resetcause::External)
    }
    #[doc = "Watchdog reset"]
    #[inline(always)]
    pub fn watchdog(self) -> &'a mut crate::W<REG> {
        self.variant(Resetcause::Watchdog)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wakeupcause(&self) -> WakeupcauseR {
        WakeupcauseR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn resetcause(&self) -> ResetcauseR {
        ResetcauseR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupcause(&mut self) -> WakeupcauseW<PmucauseSpec> {
        WakeupcauseW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn resetcause(&mut self) -> ResetcauseW<PmucauseSpec> {
        ResetcauseW::new(self, 8)
    }
}
#[doc = "PMU Cause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmucause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmucause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucauseSpec;
impl crate::RegisterSpec for PmucauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucause::R`](R) reader structure"]
impl crate::Readable for PmucauseSpec {}
#[doc = "`write(|w| ..)` method takes [`pmucause::W`](W) writer structure"]
impl crate::Writable for PmucauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pmucause to value 0"]
impl crate::Resettable for PmucauseSpec {
    const RESET_VALUE: u32 = 0;
}
