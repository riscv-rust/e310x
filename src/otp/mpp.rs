#[doc = "Register `mpp` reader"]
pub type R = crate::R<MppSpec>;
#[doc = "Register `mpp` writer"]
pub type W = crate::W<MppSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP write-voltage charge pump control\n\nYou can [`read`](crate::Reg::read) this register and get [`mpp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MppSpec;
impl crate::RegisterSpec for MppSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpp::R`](R) reader structure"]
impl crate::Readable for MppSpec {}
#[doc = "`write(|w| ..)` method takes [`mpp::W`](W) writer structure"]
impl crate::Writable for MppSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mpp to value 0"]
impl crate::Resettable for MppSpec {
    const RESET_VALUE: u32 = 0;
}
