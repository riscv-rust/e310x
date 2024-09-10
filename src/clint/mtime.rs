#[doc = "Register `mtime` reader"]
pub type R = crate::R<MtimeSpec>;
#[doc = "Register `mtime` writer"]
pub type W = crate::W<MtimeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimeSpec;
impl crate::RegisterSpec for MtimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`mtime::W`](W) writer structure"]
impl crate::Writable for MtimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtime to value 0"]
impl crate::Resettable for MtimeSpec {
    const RESET_VALUE: u32 = 0;
}
