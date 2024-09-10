#[doc = "Register `rtccmp` reader"]
pub type R = crate::R<RtccmpSpec>;
#[doc = "Register `rtccmp` writer"]
pub type W = crate::W<RtccmpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RTC Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccmpSpec;
impl crate::RegisterSpec for RtccmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccmp::R`](R) reader structure"]
impl crate::Readable for RtccmpSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccmp::W`](W) writer structure"]
impl crate::Writable for RtccmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rtccmp to value 0"]
impl crate::Resettable for RtccmpSpec {
    const RESET_VALUE: u32 = 0;
}
