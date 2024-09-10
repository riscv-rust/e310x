#[doc = "Register `csdef` reader"]
pub type R = crate::R<CsdefSpec>;
#[doc = "Register `csdef` writer"]
pub type W = crate::W<CsdefSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Chip Select Default Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csdef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsdefSpec;
impl crate::RegisterSpec for CsdefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csdef::R`](R) reader structure"]
impl crate::Readable for CsdefSpec {}
#[doc = "`write(|w| ..)` method takes [`csdef::W`](W) writer structure"]
impl crate::Writable for CsdefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets csdef to value 0xffff"]
impl crate::Resettable for CsdefSpec {
    const RESET_VALUE: u32 = 0xffff;
}
