#[doc = "Register `pmusleeppm[%s]` reader"]
pub struct R(crate::R<PMUSLEEPPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUSLEEPPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUSLEEPPM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUSLEEPPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pmusleeppm[%s]` writer"]
pub struct W(crate::W<PMUSLEEPPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUSLEEPPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMUSLEEPPM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUSLEEPPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `isolate` reader - "]
pub struct ISOLATE_R(crate::FieldReader<bool, bool>);
impl ISOLATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOLATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOLATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `isolate` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `hfclkrst` reader - "]
pub struct HFCLKRST_R(crate::FieldReader<bool, bool>);
impl HFCLKRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFCLKRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFCLKRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hfclkrst` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `corerst` reader - "]
pub struct CORERST_R(crate::FieldReader<bool, bool>);
impl CORERST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORERST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORERST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corerst` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `pmu_out_1_en` reader - "]
pub struct PMU_OUT_1_EN_R(crate::FieldReader<bool, bool>);
impl PMU_OUT_1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMU_OUT_1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_OUT_1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pmu_out_1_en` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `pmu_out_0_en` reader - "]
pub struct PMU_OUT_0_EN_R(crate::FieldReader<bool, bool>);
impl PMU_OUT_0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMU_OUT_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_OUT_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pmu_out_0_en` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `delay` reader - "]
pub struct DELAY_R(crate::FieldReader<u8, u8>);
impl DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `delay` writer - "]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Sleep Program Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmusleeppm](index.html) module"]
pub struct PMUSLEEPPM_SPEC;
impl crate::RegisterSpec for PMUSLEEPPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmusleeppm::R](R) reader structure"]
impl crate::Readable for PMUSLEEPPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmusleeppm::W](W) writer structure"]
impl crate::Writable for PMUSLEEPPM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pmusleeppm[%s]
to value 0"]
impl crate::Resettable for PMUSLEEPPM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
