#[doc = "Register `cmp1` reader"]
pub type R = crate::R<CMP1_SPEC>;
#[doc = "Register `cmp1` writer"]
pub type W = crate::W<CMP1_SPEC>;
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<CMP1_SPEC> {
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
#[doc = "Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1_SPEC;
impl crate::RegisterSpec for CMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1::R`](R) reader structure"]
impl crate::Readable for CMP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp1::W`](W) writer structure"]
impl crate::Writable for CMP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cmp1 to value 0"]
impl crate::Resettable for CMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
