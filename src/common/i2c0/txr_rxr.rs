#[doc = "Register `txr_rxr` reader"]
pub struct R(crate::R<TXR_RXR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXR_RXR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXR_RXR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXR_RXR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `txr_rxr` writer"]
pub struct W(crate::W<TXR_RXR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXR_RXR_SPEC>;
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
impl From<crate::W<TXR_RXR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXR_RXR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data` reader - "]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `data` writer - "]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit register / Receive register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txr_rxr](index.html) module"]
pub struct TXR_RXR_SPEC;
impl crate::RegisterSpec for TXR_RXR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txr_rxr::R](R) reader structure"]
impl crate::Readable for TXR_RXR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txr_rxr::W](W) writer structure"]
impl crate::Writable for TXR_RXR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets txr_rxr to value 0"]
impl crate::Resettable for TXR_RXR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
