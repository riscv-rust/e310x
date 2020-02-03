#[doc = "Reader of register pmusleeppm[%s]"]
pub type R = crate::R<u32, super::PMUSLEEPPM>;
#[doc = "Writer for register pmusleeppm[%s]"]
pub type W = crate::W<u32, super::PMUSLEEPPM>;
#[doc = "Register pmusleeppm[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::PMUSLEEPPM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `isolate`"]
pub type ISOLATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `isolate`"]
pub struct ISOLATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOLATE_W<'a> {
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
#[doc = "Reader of field `hfclkrst`"]
pub type HFCLKRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hfclkrst`"]
pub struct HFCLKRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKRST_W<'a> {
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
#[doc = "Reader of field `corerst`"]
pub type CORERST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `corerst`"]
pub struct CORERST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORERST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `pmu_out_1_en`"]
pub type PMU_OUT_1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pmu_out_1_en`"]
pub struct PMU_OUT_1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_OUT_1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `pmu_out_0_en`"]
pub type PMU_OUT_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pmu_out_0_en`"]
pub struct PMU_OUT_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_OUT_0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `delay`"]
pub type DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `delay`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn isolate(&self) -> ISOLATE_R {
        ISOLATE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hfclkrst(&self) -> HFCLKRST_R {
        HFCLKRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn corerst(&self) -> CORERST_R {
        CORERST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmu_out_1_en(&self) -> PMU_OUT_1_EN_R {
        PMU_OUT_1_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmu_out_0_en(&self) -> PMU_OUT_0_EN_R {
        PMU_OUT_0_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn isolate(&mut self) -> ISOLATE_W {
        ISOLATE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hfclkrst(&mut self) -> HFCLKRST_W {
        HFCLKRST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn corerst(&mut self) -> CORERST_W {
        CORERST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmu_out_1_en(&mut self) -> PMU_OUT_1_EN_W {
        PMU_OUT_1_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmu_out_0_en(&mut self) -> PMU_OUT_0_EN_W {
        PMU_OUT_0_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
}
