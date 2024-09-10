#[doc = "Register `wdogkey` writer"]
pub type W = crate::W<WdogkeySpec>;
impl core::fmt::Debug for crate::generic::Reg<WdogkeySpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Watchdog Key Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogkeySpec;
impl crate::RegisterSpec for WdogkeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdogkey::W`](W) writer structure"]
impl crate::Writable for WdogkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdogkey to value 0x0051_f15e"]
impl crate::Resettable for WdogkeySpec {
    const RESET_VALUE: u32 = 0x0051_f15e;
}
