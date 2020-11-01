#[doc = "Reader of register delay0"]
pub type R = crate::R<u32, super::DELAY0>;
#[doc = "Writer for register delay0"]
pub type W = crate::W<u32, super::DELAY0>;
#[doc = "Register delay0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DELAY0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sckcs`"]
pub type SCKCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sckcs`"]
pub struct SCKCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cssck`"]
pub type CSSCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cssck`"]
pub struct CSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&self) -> SCKCS_R {
        SCKCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&mut self) -> SCKCS_W {
        SCKCS_W { w: self }
    }
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&mut self) -> CSSCK_W {
        CSSCK_W { w: self }
    }
}
