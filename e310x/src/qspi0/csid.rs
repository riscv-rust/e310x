#[doc = "Register `csid` reader"]
pub type R = crate::R<CsidSpec>;
#[doc = "Register `csid` writer"]
pub type W = crate::W<CsidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Chip Select ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsidSpec;
impl crate::RegisterSpec for CsidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csid::R`](R) reader structure"]
impl crate::Readable for CsidSpec {}
#[doc = "`write(|w| ..)` method takes [`csid::W`](W) writer structure"]
impl crate::Writable for CsidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets csid to value 0"]
impl crate::Resettable for CsidSpec {
    const RESET_VALUE: u32 = 0;
}
