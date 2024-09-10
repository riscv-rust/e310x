#[doc = "Register `wdogcmp` reader"]
pub type R = crate::R<WdogcmpSpec>;
#[doc = "Register `wdogcmp` writer"]
pub type W = crate::W<WdogcmpSpec>;
#[doc = "Field `value` reader - "]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `value` writer - "]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<WdogcmpSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Watchdog Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogcmpSpec;
impl crate::RegisterSpec for WdogcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcmp::R`](R) reader structure"]
impl crate::Readable for WdogcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogcmp::W`](W) writer structure"]
impl crate::Writable for WdogcmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdogcmp to value 0"]
impl crate::Resettable for WdogcmpSpec {
    const RESET_VALUE: u32 = 0;
}
