#[doc = "Register `pmusleeppm[%s]` reader"]
pub type R = crate::R<PmusleeppmSpec>;
#[doc = "Register `pmusleeppm[%s]` writer"]
pub type W = crate::W<PmusleeppmSpec>;
#[doc = "Field `delay` reader - "]
pub type DelayR = crate::FieldReader;
#[doc = "Field `delay` writer - "]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pmu_out_0_en` reader - "]
pub type PmuOut0EnR = crate::BitReader;
#[doc = "Field `pmu_out_0_en` writer - "]
pub type PmuOut0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmu_out_1_en` reader - "]
pub type PmuOut1EnR = crate::BitReader;
#[doc = "Field `pmu_out_1_en` writer - "]
pub type PmuOut1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `corerst` reader - "]
pub type CorerstR = crate::BitReader;
#[doc = "Field `corerst` writer - "]
pub type CorerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hfclkrst` reader - "]
pub type HfclkrstR = crate::BitReader;
#[doc = "Field `hfclkrst` writer - "]
pub type HfclkrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isolate` reader - "]
pub type IsolateR = crate::BitReader;
#[doc = "Field `isolate` writer - "]
pub type IsolateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmu_out_0_en(&self) -> PmuOut0EnR {
        PmuOut0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmu_out_1_en(&self) -> PmuOut1EnR {
        PmuOut1EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn corerst(&self) -> CorerstR {
        CorerstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hfclkrst(&self) -> HfclkrstR {
        HfclkrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn isolate(&self) -> IsolateR {
        IsolateR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DelayW<PmusleeppmSpec> {
        DelayW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_out_0_en(&mut self) -> PmuOut0EnW<PmusleeppmSpec> {
        PmuOut0EnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_out_1_en(&mut self) -> PmuOut1EnW<PmusleeppmSpec> {
        PmuOut1EnW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn corerst(&mut self) -> CorerstW<PmusleeppmSpec> {
        CorerstW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hfclkrst(&mut self) -> HfclkrstW<PmusleeppmSpec> {
        HfclkrstW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn isolate(&mut self) -> IsolateW<PmusleeppmSpec> {
        IsolateW::new(self, 9)
    }
}
#[doc = "PMU Sleep Program Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`pmusleeppm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmusleeppm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmusleeppmSpec;
impl crate::RegisterSpec for PmusleeppmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmusleeppm::R`](R) reader structure"]
impl crate::Readable for PmusleeppmSpec {}
#[doc = "`write(|w| ..)` method takes [`pmusleeppm::W`](W) writer structure"]
impl crate::Writable for PmusleeppmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pmusleeppm[%s]
to value 0"]
impl crate::Resettable for PmusleeppmSpec {
    const RESET_VALUE: u32 = 0;
}
