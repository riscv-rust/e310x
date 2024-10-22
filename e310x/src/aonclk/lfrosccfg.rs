#[doc = "Register `lfrosccfg` reader"]
pub type R = crate::R<LfrosccfgSpec>;
#[doc = "Register `lfrosccfg` writer"]
pub type W = crate::W<LfrosccfgSpec>;
#[doc = "Field `div` reader - "]
pub type DivR = crate::FieldReader;
#[doc = "Field `div` writer - "]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `trim` reader - "]
pub type TrimR = crate::FieldReader;
#[doc = "Field `trim` writer - "]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `enable` reader - "]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ready` reader - "]
pub type ReadyR = crate::BitReader;
#[doc = "Field `ready` writer - "]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<LfrosccfgSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<LfrosccfgSpec> {
        TrimW::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<LfrosccfgSpec> {
        EnableW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<LfrosccfgSpec> {
        ReadyW::new(self, 31)
    }
}
#[doc = "AON Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrosccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrosccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfrosccfgSpec;
impl crate::RegisterSpec for LfrosccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrosccfg::R`](R) reader structure"]
impl crate::Readable for LfrosccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lfrosccfg::W`](W) writer structure"]
impl crate::Writable for LfrosccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lfrosccfg to value 0"]
impl crate::Resettable for LfrosccfgSpec {
    const RESET_VALUE: u32 = 0;
}
