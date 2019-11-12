#[doc = "Reader of register pmuie"]
pub type R = crate::R<u32, super::PMUIE>;
#[doc = "Writer for register pmuie"]
pub type W = crate::W<u32, super::PMUIE>;
#[doc = "Register pmuie `reset()`'s with value 0"]
impl crate::ResetValue for super::PMUIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `awakeup`"]
pub type AWAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `awakeup`"]
pub struct AWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> AWAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `dwakeup`"]
pub type DWAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dwakeup`"]
pub struct DWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DWAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `rtc`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rtc`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&self) -> AWAKEUP_R {
        AWAKEUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&self) -> DWAKEUP_R {
        DWAKEUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&mut self) -> AWAKEUP_W {
        AWAKEUP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&mut self) -> DWAKEUP_W {
        DWAKEUP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
