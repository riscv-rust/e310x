#[doc = "Register `prer_lo` reader"]
pub struct R(crate::R<PRER_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRER_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRER_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRER_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prer_lo` writer"]
pub struct W(crate::W<PRER_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRER_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRER_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRER_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRER_LO_SPEC, u8, u8, 8, O>;
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
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Prescale register lo-byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer_lo](index.html) module"]
pub struct PRER_LO_SPEC;
impl crate::RegisterSpec for PRER_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prer_lo::R](R) reader structure"]
impl crate::Readable for PRER_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prer_lo::W](W) writer structure"]
impl crate::Writable for PRER_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets prer_lo to value 0"]
impl crate::Resettable for PRER_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
