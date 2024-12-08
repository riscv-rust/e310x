#[doc = "Register `pmusleep` writer"]
pub type W = crate::W<PmusleepSpec>;
#[doc = "Field `sleep` writer - "]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<PmusleepSpec> {
        SleepW::new(self, 0)
    }
}
#[doc = "PMU Sleep Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmusleep::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmusleepSpec;
impl crate::RegisterSpec for PmusleepSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmusleep::W`](W) writer structure"]
impl crate::Writable for PmusleepSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pmusleep to value 0"]
impl crate::Resettable for PmusleepSpec {
    const RESET_VALUE: u32 = 0;
}
