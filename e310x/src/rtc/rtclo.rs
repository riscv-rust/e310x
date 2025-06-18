#[doc = "Register `rtclo` reader"]
pub type R = crate::R<RtcloSpec>;
#[doc = "Register `rtclo` writer"]
pub type W = crate::W<RtcloSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RTC Counter Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtclo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtclo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcloSpec;
impl crate::RegisterSpec for RtcloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtclo::R`](R) reader structure"]
impl crate::Readable for RtcloSpec {}
#[doc = "`write(|w| ..)` method takes [`rtclo::W`](W) writer structure"]
impl crate::Writable for RtcloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rtclo to value 0"]
impl crate::Resettable for RtcloSpec {}
