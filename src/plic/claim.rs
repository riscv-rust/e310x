#[doc = "Register `claim` reader"]
pub type R = crate::R<ClaimSpec>;
#[doc = "Register `claim` writer"]
pub type W = crate::W<ClaimSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimSpec;
impl crate::RegisterSpec for ClaimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claim::R`](R) reader structure"]
impl crate::Readable for ClaimSpec {}
#[doc = "`write(|w| ..)` method takes [`claim::W`](W) writer structure"]
impl crate::Writable for ClaimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets claim to value 0"]
impl crate::Resettable for ClaimSpec {
    const RESET_VALUE: u32 = 0;
}
