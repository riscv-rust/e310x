#[doc = "Register `delay1` reader"]
pub struct R(crate::R<DELAY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `delay1` writer"]
pub struct W(crate::W<DELAY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY1_SPEC>;
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
impl From<crate::W<DELAY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `intercs` reader - Minimum CS inactive time"]
pub type INTERCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `intercs` writer - Minimum CS inactive time"]
pub type INTERCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY1_SPEC, u8, u8, 8, O>;
#[doc = "Field `interxfr` reader - Maximum interframe delay"]
pub type INTERXFR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `interxfr` writer - Maximum interframe delay"]
pub type INTERXFR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&self) -> INTERCS_R {
        INTERCS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&self) -> INTERXFR_R {
        INTERXFR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&mut self) -> INTERCS_W<0> {
        INTERCS_W::new(self)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&mut self) -> INTERXFR_W<16> {
        INTERXFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay1](index.html) module"]
pub struct DELAY1_SPEC;
impl crate::RegisterSpec for DELAY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay1::R](R) reader structure"]
impl crate::Readable for DELAY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay1::W](W) writer structure"]
impl crate::Writable for DELAY1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets delay1 to value 0x01"]
impl crate::Resettable for DELAY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
