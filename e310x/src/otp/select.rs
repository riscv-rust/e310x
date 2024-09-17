#[doc = "Register `select` reader"]
pub type R = crate::R<SelectSpec>;
#[doc = "Register `select` writer"]
pub type W = crate::W<SelectSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP device chip-select signal\n\nYou can [`read`](crate::Reg::read) this register and get [`select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelectSpec;
impl crate::RegisterSpec for SelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`select::R`](R) reader structure"]
impl crate::Readable for SelectSpec {}
#[doc = "`write(|w| ..)` method takes [`select::W`](W) writer structure"]
impl crate::Writable for SelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets select to value 0"]
impl crate::Resettable for SelectSpec {
    const RESET_VALUE: u32 = 0;
}
