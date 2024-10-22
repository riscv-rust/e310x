#[doc = "Register `cmp0` reader"]
pub type R = crate::R<Cmp0Spec>;
#[doc = "Register `cmp0` writer"]
pub type W = crate::W<Cmp0Spec>;
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
    pub fn value(&mut self) -> ValueW<Cmp0Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp0Spec;
impl crate::RegisterSpec for Cmp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0::R`](R) reader structure"]
impl crate::Readable for Cmp0Spec {}
#[doc = "`write(|w| ..)` method takes [`cmp0::W`](W) writer structure"]
impl crate::Writable for Cmp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmp0 to value 0"]
impl crate::Resettable for Cmp0Spec {
    const RESET_VALUE: u32 = 0;
}
