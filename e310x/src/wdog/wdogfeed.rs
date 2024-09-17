#[doc = "Register `wdogfeed` reader"]
pub type R = crate::R<WdogfeedSpec>;
#[doc = "Register `wdogfeed` writer"]
pub type W = crate::W<WdogfeedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog Feed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogfeed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogfeed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogfeedSpec;
impl crate::RegisterSpec for WdogfeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogfeed::R`](R) reader structure"]
impl crate::Readable for WdogfeedSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogfeed::W`](W) writer structure"]
impl crate::Writable for WdogfeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdogfeed to value 0"]
impl crate::Resettable for WdogfeedSpec {
    const RESET_VALUE: u32 = 0;
}
