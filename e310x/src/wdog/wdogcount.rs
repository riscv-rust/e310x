#[doc = "Register `wdogcount` reader"]
pub type R = crate::R<WdogcountSpec>;
#[doc = "Register `wdogcount` writer"]
pub type W = crate::W<WdogcountSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogcountSpec;
impl crate::RegisterSpec for WdogcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcount::R`](R) reader structure"]
impl crate::Readable for WdogcountSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogcount::W`](W) writer structure"]
impl crate::Writable for WdogcountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdogcount to value 0"]
impl crate::Resettable for WdogcountSpec {
    const RESET_VALUE: u32 = 0;
}
