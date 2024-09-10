#[doc = "Register `mtimecmp` reader"]
pub type R = crate::R<MtimecmpSpec>;
#[doc = "Register `mtimecmp` writer"]
pub type W = crate::W<MtimecmpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Hart 0 time comparator register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimecmpSpec;
impl crate::RegisterSpec for MtimecmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp::R`](R) reader structure"]
impl crate::Readable for MtimecmpSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp::W`](W) writer structure"]
impl crate::Writable for MtimecmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtimecmp to value 0"]
impl crate::Resettable for MtimecmpSpec {
    const RESET_VALUE: u32 = 0;
}
