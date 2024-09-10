#[doc = "Register `pmukey` reader"]
pub type R = crate::R<PmukeySpec>;
#[doc = "Register `pmukey` writer"]
pub type W = crate::W<PmukeySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PMU Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmukey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmukey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmukeySpec;
impl crate::RegisterSpec for PmukeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmukey::R`](R) reader structure"]
impl crate::Readable for PmukeySpec {}
#[doc = "`write(|w| ..)` method takes [`pmukey::W`](W) writer structure"]
impl crate::Writable for PmukeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pmukey to value 0"]
impl crate::Resettable for PmukeySpec {
    const RESET_VALUE: u32 = 0;
}
