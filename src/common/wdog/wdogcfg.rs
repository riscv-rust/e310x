#[doc = "Reader of register wdogcfg"]
pub type R = crate::R<u32, super::WDOGCFG>;
#[doc = "Writer for register wdogcfg"]
pub type W = crate::W<u32, super::WDOGCFG>;
#[doc = "Register wdogcfg `reset()`'s with value 0"]
impl crate::ResetValue for super::WDOGCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cmpip`"]
pub type CMPIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmpip`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `encoreawake`"]
pub type ENCOREAWAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `encoreawake`"]
pub struct ENCOREAWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCOREAWAKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `enalways`"]
pub type ENALWAYS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enalways`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `zerocmp`"]
pub type ZEROCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `zerocmp`"]
pub struct ZEROCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `rsten`"]
pub type RSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsten`"]
pub struct RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTEN_W<'a> {
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
#[doc = "Reader of field `scale`"]
pub type SCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `scale`"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&self) -> CMPIP_R {
        CMPIP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn encoreawake(&self) -> ENCOREAWAKE_R {
        ENCOREAWAKE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZEROCMP_R {
        ZEROCMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn encoreawake(&mut self) -> ENCOREAWAKE_W {
        ENCOREAWAKE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W {
        ENALWAYS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&mut self) -> ZEROCMP_W {
        ZEROCMP_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W {
        RSTEN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
}
