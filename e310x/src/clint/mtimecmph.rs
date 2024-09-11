#[doc = "Register `mtimecmph` reader"]
pub type R = crate::R<MtimecmphSpec>;
#[doc = "Register `mtimecmph` writer"]
pub type W = crate::W<MtimecmphSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Hart 0 time comparator register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmph::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimecmphSpec;
impl crate::RegisterSpec for MtimecmphSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmph::R`](R) reader structure"]
impl crate::Readable for MtimecmphSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmph::W`](W) writer structure"]
impl crate::Writable for MtimecmphSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtimecmph to value 0"]
impl crate::Resettable for MtimecmphSpec {
    const RESET_VALUE: u32 = 0;
}
