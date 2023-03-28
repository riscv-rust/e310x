#[doc = "Register `txmark` reader"]
pub struct R(crate::R<TXMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `txmark` writer"]
pub struct W(crate::W<TXMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXMARK_SPEC>;
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
impl From<crate::W<TXMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txmark` reader - Transmit watermark"]
pub type TXMARK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `txmark` writer - Transmit watermark"]
pub type TXMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXMARK_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    pub fn txmark(&self) -> TXMARK_R {
        TXMARK_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    pub fn txmark(&mut self) -> TXMARK_W<0> {
        TXMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmark](index.html) module"]
pub struct TXMARK_SPEC;
impl crate::RegisterSpec for TXMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmark::R](R) reader structure"]
impl crate::Readable for TXMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txmark::W](W) writer structure"]
impl crate::Writable for TXMARK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets txmark to value 0"]
impl crate::Resettable for TXMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
