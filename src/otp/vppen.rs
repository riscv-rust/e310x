#[doc = "Register `vppen` reader"]
pub type R = crate::R<VppenSpec>;
#[doc = "Register `vppen` writer"]
pub type W = crate::W<VppenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP write-voltage enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vppen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vppen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VppenSpec;
impl crate::RegisterSpec for VppenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vppen::R`](R) reader structure"]
impl crate::Readable for VppenSpec {}
#[doc = "`write(|w| ..)` method takes [`vppen::W`](W) writer structure"]
impl crate::Writable for VppenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vppen to value 0"]
impl crate::Resettable for VppenSpec {
    const RESET_VALUE: u32 = 0;
}
