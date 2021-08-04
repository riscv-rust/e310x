#[doc = "Register `plloutdiv` reader"]
pub struct R(crate::R<PLLOUTDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLOUTDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLOUTDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLOUTDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `plloutdiv` writer"]
pub struct W(crate::W<PLLOUTDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLOUTDIV_SPEC>;
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
impl From<crate::W<PLLOUTDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLOUTDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `divby1` reader - "]
pub struct DIVBY1_R(crate::FieldReader<bool, bool>);
impl DIVBY1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVBY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVBY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `divby1` writer - "]
pub struct DIVBY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVBY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `div` reader - "]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `div` writer - "]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&self) -> DIVBY1_R {
        DIVBY1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&mut self) -> DIVBY1_W {
        DIVBY1_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plloutdiv](index.html) module"]
pub struct PLLOUTDIV_SPEC;
impl crate::RegisterSpec for PLLOUTDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plloutdiv::R](R) reader structure"]
impl crate::Readable for PLLOUTDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plloutdiv::W](W) writer structure"]
impl crate::Writable for PLLOUTDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets plloutdiv to value 0x0100"]
impl crate::Resettable for PLLOUTDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
