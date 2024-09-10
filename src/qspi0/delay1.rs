#[doc = "Register `delay1` reader"]
pub type R = crate::R<Delay1Spec>;
#[doc = "Register `delay1` writer"]
pub type W = crate::W<Delay1Spec>;
#[doc = "Field `intercs` reader - Minimum CS inactive time"]
pub type IntercsR = crate::FieldReader;
#[doc = "Field `intercs` writer - Minimum CS inactive time"]
pub type IntercsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `interxfr` reader - Maximum interframe delay"]
pub type InterxfrR = crate::FieldReader;
#[doc = "Field `interxfr` writer - Maximum interframe delay"]
pub type InterxfrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&self) -> IntercsR {
        IntercsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&self) -> InterxfrR {
        InterxfrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    #[must_use]
    pub fn intercs(&mut self) -> IntercsW<Delay1Spec> {
        IntercsW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    #[must_use]
    pub fn interxfr(&mut self) -> InterxfrW<Delay1Spec> {
        InterxfrW::new(self, 16)
    }
}
#[doc = "Delay Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`delay1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Delay1Spec;
impl crate::RegisterSpec for Delay1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay1::R`](R) reader structure"]
impl crate::Readable for Delay1Spec {}
#[doc = "`write(|w| ..)` method takes [`delay1::W`](W) writer structure"]
impl crate::Writable for Delay1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets delay1 to value 0x01"]
impl crate::Resettable for Delay1Spec {
    const RESET_VALUE: u32 = 0x01;
}
