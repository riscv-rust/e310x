#[doc = "Register `rtccfg` reader"]
pub struct R(crate::R<RTCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtccfg` writer"]
pub struct W(crate::W<RTCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCFG_SPEC>;
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
impl From<crate::W<RTCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmpip` reader - "]
pub struct CMPIP_R(crate::FieldReader<bool, bool>);
impl CMPIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmpip` writer - "]
pub struct CMPIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `enalways` reader - "]
pub struct ENALWAYS_R(crate::FieldReader<bool, bool>);
impl ENALWAYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENALWAYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENALWAYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enalways` writer - "]
pub struct ENALWAYS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENALWAYS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `scale` reader - "]
pub struct SCALE_R(crate::FieldReader<u8, u8>);
impl SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `scale` writer - "]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&self) -> CMPIP_R {
        CMPIP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&mut self) -> CMPIP_W {
        CMPIP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W {
        ENALWAYS_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccfg](index.html) module"]
pub struct RTCCFG_SPEC;
impl crate::RegisterSpec for RTCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccfg::R](R) reader structure"]
impl crate::Readable for RTCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccfg::W](W) writer structure"]
impl crate::Writable for RTCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rtccfg to value 0"]
impl crate::Resettable for RTCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
