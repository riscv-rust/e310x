#[doc = "Register `output_en` reader"]
pub type R = crate::R<OutputEnSpec>;
#[doc = "Register `output_en` writer"]
pub type W = crate::W<OutputEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP device output-enable signal\n\nYou can [`read`](crate::Reg::read) this register and get [`output_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputEnSpec;
impl crate::RegisterSpec for OutputEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`output_en::R`](R) reader structure"]
impl crate::Readable for OutputEnSpec {}
#[doc = "`write(|w| ..)` method takes [`output_en::W`](W) writer structure"]
impl crate::Writable for OutputEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets output_en to value 0"]
impl crate::Resettable for OutputEnSpec {
    const RESET_VALUE: u32 = 0;
}
