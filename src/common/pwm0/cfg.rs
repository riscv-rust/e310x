#[doc = "Reader of register cfg"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register cfg"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cmp3ip`"]
pub type CMP3IP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp3ip`"]
pub struct CMP3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IP_W<'a> {
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
#[doc = "Reader of field `cmp2ip`"]
pub type CMP2IP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp2ip`"]
pub struct CMP2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `cmp1ip`"]
pub type CMP1IP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp1ip`"]
pub struct CMP1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `cmp0ip`"]
pub type CMP0IP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp0ip`"]
pub struct CMP0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0IP_W<'a> {
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
#[doc = "Reader of field `cmp3gang`"]
pub type CMP3GANG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp3gang`"]
pub struct CMP3GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3GANG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `cmp2gang`"]
pub type CMP2GANG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cmp2gang`"]
pub struct CMP2GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2GANG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 26)) | (((value as u32) & 0x07ff) << 26);
        self.w
    }
}
#[doc = "Reader of field `cmp1gang`"]
pub type CMP1GANG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp1gang`"]
pub struct CMP1GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1GANG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `cmp0gang`"]
pub type CMP0GANG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp0gang`"]
pub struct CMP0GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0GANG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `cmp3center`"]
pub type CMP3CENTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp3center`"]
pub struct CMP3CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `cmp2center`"]
pub type CMP2CENTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp2center`"]
pub struct CMP2CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `cmp1center`"]
pub type CMP1CENTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp1center`"]
pub struct CMP1CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `cmp0center`"]
pub type CMP0CENTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmp0center`"]
pub struct CMP0CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `enoneshot`"]
pub type ENONESHOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enoneshot`"]
pub struct ENONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENONESHOT_W<'a> {
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
#[doc = "Reader of field `deglitch`"]
pub type DEGLITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `deglitch`"]
pub struct DEGLITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
#[doc = "Reader of field `sticky`"]
pub type STICKY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sticky`"]
pub struct STICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> STICKY_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&self) -> CMP3IP_R {
        CMP3IP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&self) -> CMP2IP_R {
        CMP2IP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&self) -> CMP1IP_R {
        CMP1IP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&self) -> CMP0IP_R {
        CMP0IP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&self) -> CMP3GANG_R {
        CMP3GANG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&self) -> CMP2GANG_R {
        CMP2GANG_R::new(((self.bits >> 26) & 0x07ff) as u16)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&self) -> CMP1GANG_R {
        CMP1GANG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&self) -> CMP0GANG_R {
        CMP0GANG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&self) -> CMP3CENTER_R {
        CMP3CENTER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&self) -> CMP2CENTER_R {
        CMP2CENTER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&self) -> CMP1CENTER_R {
        CMP1CENTER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&self) -> CMP0CENTER_R {
        CMP0CENTER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&self) -> ENONESHOT_R {
        ENONESHOT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&self) -> DEGLITCH_R {
        DEGLITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZEROCMP_R {
        ZEROCMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&self) -> STICKY_R {
        STICKY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&mut self) -> CMP3IP_W {
        CMP3IP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&mut self) -> CMP2IP_W {
        CMP2IP_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&mut self) -> CMP1IP_W {
        CMP1IP_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&mut self) -> CMP0IP_W {
        CMP0IP_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&mut self) -> CMP3GANG_W {
        CMP3GANG_W { w: self }
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&mut self) -> CMP2GANG_W {
        CMP2GANG_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&mut self) -> CMP1GANG_W {
        CMP1GANG_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&mut self) -> CMP0GANG_W {
        CMP0GANG_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&mut self) -> CMP3CENTER_W {
        CMP3CENTER_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&mut self) -> CMP2CENTER_W {
        CMP2CENTER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&mut self) -> CMP1CENTER_W {
        CMP1CENTER_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&mut self) -> CMP0CENTER_W {
        CMP0CENTER_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&mut self) -> ENONESHOT_W {
        ENONESHOT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W {
        ENALWAYS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&mut self) -> DEGLITCH_W {
        DEGLITCH_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&mut self) -> ZEROCMP_W {
        ZEROCMP_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&mut self) -> STICKY_W {
        STICKY_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
}
