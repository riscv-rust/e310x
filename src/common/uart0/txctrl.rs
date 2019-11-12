#[doc = "Reader of register txctrl"]
pub type R = crate::R<u32, super::TXCTRL>;
#[doc = "Writer for register txctrl"]
pub type W = crate::W<u32, super::TXCTRL>;
#[doc = "Register txctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `counter`"]
pub type COUNTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `counter`"]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `nstop`"]
pub type NSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `nstop`"]
pub struct NSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSTOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `enable`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enable`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&self) -> NSTOP_R {
        NSTOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&mut self) -> NSTOP_W {
        NSTOP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
