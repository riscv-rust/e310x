#[doc = "Register `prer_hi` reader"]
pub type R = crate::R<PRER_HI_SPEC>;
#[doc = "Register `prer_hi` writer"]
pub type W = crate::W<PRER_HI_SPEC>;
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<PRER_HI_SPEC> {
        VALUE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Prescale register hi-byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRER_HI_SPEC;
impl crate::RegisterSpec for PRER_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prer_hi::R`](R) reader structure"]
impl crate::Readable for PRER_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prer_hi::W`](W) writer structure"]
impl crate::Writable for PRER_HI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prer_hi to value 0"]
impl crate::Resettable for PRER_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
