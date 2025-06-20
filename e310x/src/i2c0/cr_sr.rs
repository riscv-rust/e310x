#[doc = "Register `cr_sr` reader"]
pub type R = crate::R<CrSrSpec>;
#[doc = "Register `cr_sr` writer"]
pub type W = crate::W<CrSrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Command register / Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSrSpec;
impl crate::RegisterSpec for CrSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_sr::R`](R) reader structure"]
impl crate::Readable for CrSrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_sr::W`](W) writer structure"]
impl crate::Writable for CrSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cr_sr to value 0"]
impl crate::Resettable for CrSrSpec {}
