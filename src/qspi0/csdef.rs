#[doc = "Register `csdef` reader"]
pub type R = crate::R<CSDEF_SPEC>;
#[doc = "Register `csdef` writer"]
pub type W = crate::W<CSDEF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CSDEF_SPEC> {
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
#[doc = "Chip Select Default Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csdef::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csdef::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSDEF_SPEC;
impl crate::RegisterSpec for CSDEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csdef::R`](R) reader structure"]
impl crate::Readable for CSDEF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csdef::W`](W) writer structure"]
impl crate::Writable for CSDEF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csdef to value 0xffff"]
impl crate::Resettable for CSDEF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
