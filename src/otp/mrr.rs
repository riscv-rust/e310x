#[doc = "Register `mrr` reader"]
pub type R = crate::R<MRR_SPEC>;
#[doc = "Register `mrr` writer"]
pub type W = crate::W<MRR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MRR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "OTP read-voltage regulator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MRR_SPEC;
impl crate::RegisterSpec for MRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrr::R`](R) reader structure"]
impl crate::Readable for MRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mrr::W`](W) writer structure"]
impl crate::Writable for MRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mrr to value 0"]
impl crate::Resettable for MRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
