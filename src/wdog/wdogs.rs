#[doc = "Register `wdogs` reader"]
pub type R = crate::R<WdogsSpec>;
#[doc = "Register `wdogs` writer"]
pub type W = crate::W<WdogsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog Scaled Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogsSpec;
impl crate::RegisterSpec for WdogsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogs::R`](R) reader structure"]
impl crate::Readable for WdogsSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogs::W`](W) writer structure"]
impl crate::Writable for WdogsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdogs to value 0"]
impl crate::Resettable for WdogsSpec {
    const RESET_VALUE: u32 = 0;
}
