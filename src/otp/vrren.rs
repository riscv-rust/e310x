#[doc = "Register `vrren` reader"]
pub type R = crate::R<VrrenSpec>;
#[doc = "Register `vrren` writer"]
pub type W = crate::W<VrrenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP read-voltage enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vrren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrrenSpec;
impl crate::RegisterSpec for VrrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrren::R`](R) reader structure"]
impl crate::Readable for VrrenSpec {}
#[doc = "`write(|w| ..)` method takes [`vrren::W`](W) writer structure"]
impl crate::Writable for VrrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vrren to value 0"]
impl crate::Resettable for VrrenSpec {
    const RESET_VALUE: u32 = 0;
}
