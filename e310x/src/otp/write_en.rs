#[doc = "Register `write_en` reader"]
pub type R = crate::R<WriteEnSpec>;
#[doc = "Register `write_en` writer"]
pub type W = crate::W<WriteEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP device write-enable signal\n\nYou can [`read`](crate::Reg::read) this register and get [`write_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteEnSpec;
impl crate::RegisterSpec for WriteEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_en::R`](R) reader structure"]
impl crate::Readable for WriteEnSpec {}
#[doc = "`write(|w| ..)` method takes [`write_en::W`](W) writer structure"]
impl crate::Writable for WriteEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets write_en to value 0"]
impl crate::Resettable for WriteEnSpec {
    const RESET_VALUE: u32 = 0;
}
