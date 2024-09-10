#[doc = "Register `rtchi` reader"]
pub type R = crate::R<RtchiSpec>;
#[doc = "Register `rtchi` writer"]
pub type W = crate::W<RtchiSpec>;
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
    pub fn value(&mut self) -> ValueW<RtchiSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "RTC Counter High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtchi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtchi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtchiSpec;
impl crate::RegisterSpec for RtchiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtchi::R`](R) reader structure"]
impl crate::Readable for RtchiSpec {}
#[doc = "`write(|w| ..)` method takes [`rtchi::W`](W) writer structure"]
impl crate::Writable for RtchiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rtchi to value 0"]
impl crate::Resettable for RtchiSpec {
    const RESET_VALUE: u32 = 0;
}
