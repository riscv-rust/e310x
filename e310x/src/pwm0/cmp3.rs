#[doc = "Register `cmp3` reader"]
pub type R = crate::R<Cmp3Spec>;
#[doc = "Register `cmp3` writer"]
pub type W = crate::W<Cmp3Spec>;
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
    pub fn value(&mut self) -> ValueW<Cmp3Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp3Spec;
impl crate::RegisterSpec for Cmp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3::R`](R) reader structure"]
impl crate::Readable for Cmp3Spec {}
#[doc = "`write(|w| ..)` method takes [`cmp3::W`](W) writer structure"]
impl crate::Writable for Cmp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmp3 to value 0"]
impl crate::Resettable for Cmp3Spec {
    const RESET_VALUE: u32 = 0;
}
