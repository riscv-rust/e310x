#[doc = "Register `pmuwakepm[%s]` reader"]
pub struct R(crate::R<PMUWAKEPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUWAKEPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUWAKEPM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUWAKEPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pmuwakepm[%s]` writer"]
pub struct W(crate::W<PMUWAKEPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUWAKEPM_SPEC>;
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
impl From<crate::W<PMUWAKEPM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUWAKEPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `delay` reader - "]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `delay` writer - "]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMUWAKEPM_SPEC, u8, u8, 4, O>;
#[doc = "Field `pmu_out_0_en` reader - "]
pub type PMU_OUT_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `pmu_out_0_en` writer - "]
pub type PMU_OUT_0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUWAKEPM_SPEC, bool, O>;
#[doc = "Field `pmu_out_1_en` reader - "]
pub type PMU_OUT_1_EN_R = crate::BitReader<bool>;
#[doc = "Field `pmu_out_1_en` writer - "]
pub type PMU_OUT_1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUWAKEPM_SPEC, bool, O>;
#[doc = "Field `corerst` reader - "]
pub type CORERST_R = crate::BitReader<bool>;
#[doc = "Field `corerst` writer - "]
pub type CORERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUWAKEPM_SPEC, bool, O>;
#[doc = "Field `hfclkrst` reader - "]
pub type HFCLKRST_R = crate::BitReader<bool>;
#[doc = "Field `hfclkrst` writer - "]
pub type HFCLKRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUWAKEPM_SPEC, bool, O>;
#[doc = "Field `isolate` reader - "]
pub type ISOLATE_R = crate::BitReader<bool>;
#[doc = "Field `isolate` writer - "]
pub type ISOLATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUWAKEPM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmu_out_0_en(&self) -> PMU_OUT_0_EN_R {
        PMU_OUT_0_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmu_out_1_en(&self) -> PMU_OUT_1_EN_R {
        PMU_OUT_1_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn corerst(&self) -> CORERST_R {
        CORERST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hfclkrst(&self) -> HFCLKRST_R {
        HFCLKRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn isolate(&self) -> ISOLATE_R {
        ISOLATE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<0> {
        DELAY_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmu_out_0_en(&mut self) -> PMU_OUT_0_EN_W<4> {
        PMU_OUT_0_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmu_out_1_en(&mut self) -> PMU_OUT_1_EN_W<5> {
        PMU_OUT_1_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn corerst(&mut self) -> CORERST_W<7> {
        CORERST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hfclkrst(&mut self) -> HFCLKRST_W<8> {
        HFCLKRST_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn isolate(&mut self) -> ISOLATE_W<9> {
        ISOLATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Wake Program Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmuwakepm](index.html) module"]
pub struct PMUWAKEPM_SPEC;
impl crate::RegisterSpec for PMUWAKEPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmuwakepm::R](R) reader structure"]
impl crate::Readable for PMUWAKEPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmuwakepm::W](W) writer structure"]
impl crate::Writable for PMUWAKEPM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pmuwakepm[%s]
to value 0"]
impl crate::Resettable for PMUWAKEPM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
