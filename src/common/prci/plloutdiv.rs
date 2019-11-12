#[doc = "Reader of register plloutdiv"]
pub type R = crate::R<u32, super::PLLOUTDIV>;
#[doc = "Writer for register plloutdiv"]
pub type W = crate::W<u32, super::PLLOUTDIV>;
#[doc = "Register plloutdiv `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::PLLOUTDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `divby1`"]
pub type DIVBY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `divby1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `div`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `div`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
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
}
