#[doc = "Register `pmuie` reader"]
pub type R = crate::R<PMUIE_SPEC>;
#[doc = "Register `pmuie` writer"]
pub type W = crate::W<PMUIE_SPEC>;
#[doc = "Field `rtc` reader - "]
pub type RTC_R = crate::BitReader;
#[doc = "Field `rtc` writer - "]
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dwakeup` reader - "]
pub type DWAKEUP_R = crate::BitReader;
#[doc = "Field `dwakeup` writer - "]
pub type DWAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `awakeup` reader - "]
pub type AWAKEUP_R = crate::BitReader;
#[doc = "Field `awakeup` writer - "]
pub type AWAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&self) -> DWAKEUP_R {
        DWAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&self) -> AWAKEUP_R {
        AWAKEUP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<PMUIE_SPEC> {
        RTC_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dwakeup(&mut self) -> DWAKEUP_W<PMUIE_SPEC> {
        DWAKEUP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn awakeup(&mut self) -> AWAKEUP_W<PMUIE_SPEC> {
        AWAKEUP_W::new(self, 3)
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
#[doc = "PMU Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUIE_SPEC;
impl crate::RegisterSpec for PMUIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmuie::R`](R) reader structure"]
impl crate::Readable for PMUIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmuie::W`](W) writer structure"]
impl crate::Writable for PMUIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pmuie to value 0"]
impl crate::Resettable for PMUIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
