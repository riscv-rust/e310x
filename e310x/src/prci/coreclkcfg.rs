#[doc = "Register `coreclkcfg` reader"]
pub type R = crate::R<CoreclkcfgSpec>;
#[doc = "Register `coreclkcfg` writer"]
pub type W = crate::W<CoreclkcfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`coreclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coreclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreclkcfgSpec;
impl crate::RegisterSpec for CoreclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coreclkcfg::R`](R) reader structure"]
impl crate::Readable for CoreclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`coreclkcfg::W`](W) writer structure"]
impl crate::Writable for CoreclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets coreclkcfg to value 0"]
impl crate::Resettable for CoreclkcfgSpec {}
