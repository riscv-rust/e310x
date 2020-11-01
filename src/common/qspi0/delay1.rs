#[doc = "Reader of register delay1"]
pub type R = crate::R<u32, super::DELAY1>;
#[doc = "Writer for register delay1"]
pub type W = crate::W<u32, super::DELAY1>;
#[doc = "Register delay1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DELAY1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `interxfr`"]
pub type INTERXFR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `interxfr`"]
pub struct INTERXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERXFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `intercs`"]
pub type INTERCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `intercs`"]
pub struct INTERCS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&self) -> INTERXFR_R {
        INTERXFR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&self) -> INTERCS_R {
        INTERCS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&mut self) -> INTERXFR_W {
        INTERXFR_W { w: self }
    }
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&mut self) -> INTERCS_W {
        INTERCS_W { w: self }
    }
}
