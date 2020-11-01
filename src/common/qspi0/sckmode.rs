#[doc = "Reader of register sckmode"]
pub type R = crate::R<u32, super::SCKMODE>;
#[doc = "Writer for register sckmode"]
pub type W = crate::W<u32, super::SCKMODE>;
#[doc = "Register sckmode `reset()`'s with value 0"]
impl crate::ResetValue for super::SCKMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pol`"]
pub type POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pol`"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
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
#[doc = "Reader of field `pha`"]
pub type PHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pha`"]
pub struct PHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHA_W<'a> {
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
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&mut self) -> PHA_W {
        PHA_W { w: self }
    }
}
