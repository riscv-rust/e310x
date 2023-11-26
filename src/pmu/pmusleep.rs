#[doc = "Register `pmusleep` writer"]
pub type W = crate::W<PMUSLEEP_SPEC>;
#[doc = "Field `sleep` writer - "]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<PMUSLEEP_SPEC> {
        SLEEP_W::new(self, 0)
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
#[doc = "PMU Sleep Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleep::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUSLEEP_SPEC;
impl crate::RegisterSpec for PMUSLEEP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmusleep::W`](W) writer structure"]
impl crate::Writable for PMUSLEEP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pmusleep to value 0"]
impl crate::Resettable for PMUSLEEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
