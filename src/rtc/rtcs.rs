#[doc = "Register `rtcs` reader"]
pub type R = crate::R<RtcsSpec>;
#[doc = "Register `rtcs` writer"]
pub type W = crate::W<RtcsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RTC Scaled Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcsSpec;
impl crate::RegisterSpec for RtcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcs::R`](R) reader structure"]
impl crate::Readable for RtcsSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcs::W`](W) writer structure"]
impl crate::Writable for RtcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rtcs to value 0"]
impl crate::Resettable for RtcsSpec {
    const RESET_VALUE: u32 = 0;
}
