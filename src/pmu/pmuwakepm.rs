#[doc = "Register `pmuwakepm[%s]` reader"]
pub type R = crate::R<PMUWAKEPM_SPEC>;
#[doc = "Register `pmuwakepm[%s]` writer"]
pub type W = crate::W<PMUWAKEPM_SPEC>;
#[doc = "Field `delay` reader - "]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `delay` writer - "]
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pmu_out_0_en` reader - "]
pub type PMU_OUT_0_EN_R = crate::BitReader;
#[doc = "Field `pmu_out_0_en` writer - "]
pub type PMU_OUT_0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmu_out_1_en` reader - "]
pub type PMU_OUT_1_EN_R = crate::BitReader;
#[doc = "Field `pmu_out_1_en` writer - "]
pub type PMU_OUT_1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `corerst` reader - "]
pub type CORERST_R = crate::BitReader;
#[doc = "Field `corerst` writer - "]
pub type CORERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hfclkrst` reader - "]
pub type HFCLKRST_R = crate::BitReader;
#[doc = "Field `hfclkrst` writer - "]
pub type HFCLKRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isolate` reader - "]
pub type ISOLATE_R = crate::BitReader;
#[doc = "Field `isolate` writer - "]
pub type ISOLATE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<PMUWAKEPM_SPEC> {
        DELAY_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_out_0_en(&mut self) -> PMU_OUT_0_EN_W<PMUWAKEPM_SPEC> {
        PMU_OUT_0_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_out_1_en(&mut self) -> PMU_OUT_1_EN_W<PMUWAKEPM_SPEC> {
        PMU_OUT_1_EN_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn corerst(&mut self) -> CORERST_W<PMUWAKEPM_SPEC> {
        CORERST_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hfclkrst(&mut self) -> HFCLKRST_W<PMUWAKEPM_SPEC> {
        HFCLKRST_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn isolate(&mut self) -> ISOLATE_W<PMUWAKEPM_SPEC> {
        ISOLATE_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PMU Wake Program Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakepm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakepm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUWAKEPM_SPEC;
impl crate::RegisterSpec for PMUWAKEPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmuwakepm::R`](R) reader structure"]
impl crate::Readable for PMUWAKEPM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmuwakepm::W`](W) writer structure"]
impl crate::Writable for PMUWAKEPM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pmuwakepm[%s]
to value 0"]
impl crate::Resettable for PMUWAKEPM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
