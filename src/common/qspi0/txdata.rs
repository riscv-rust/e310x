#[doc = "Reader of register txdata"]
pub type R = crate::R<u32, super::TXDATA>;
#[doc = "Writer for register txdata"]
pub type W = crate::W<u32, super::TXDATA>;
#[doc = "Register txdata `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `full`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `full`"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `data`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `data`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - FIFO full flag"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Transmit data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - FIFO full flag"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bits 0:7 - Transmit data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
