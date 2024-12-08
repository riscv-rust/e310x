#[doc = "Register `prer_hi` reader"]
pub type R = crate::R<PrerHiSpec>;
#[doc = "Register `prer_hi` writer"]
pub type W = crate::W<PrerHiSpec>;
#[doc = "Field `value` reader - "]
pub type ValueR = crate::FieldReader;
#[doc = "Field `value` writer - "]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<PrerHiSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Clock Prescale register hi-byte\n\nYou can [`read`](crate::Reg::read) this register and get [`prer_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrerHiSpec;
impl crate::RegisterSpec for PrerHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prer_hi::R`](R) reader structure"]
impl crate::Readable for PrerHiSpec {}
#[doc = "`write(|w| ..)` method takes [`prer_hi::W`](W) writer structure"]
impl crate::Writable for PrerHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prer_hi to value 0"]
impl crate::Resettable for PrerHiSpec {
    const RESET_VALUE: u32 = 0;
}
