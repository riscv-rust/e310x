#[doc = "Register `mrr` reader"]
pub type R = crate::R<MrrSpec>;
#[doc = "Register `mrr` writer"]
pub type W = crate::W<MrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP read-voltage regulator control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrrSpec;
impl crate::RegisterSpec for MrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrr::R`](R) reader structure"]
impl crate::Readable for MrrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrr::W`](W) writer structure"]
impl crate::Writable for MrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mrr to value 0"]
impl crate::Resettable for MrrSpec {
    const RESET_VALUE: u32 = 0;
}
