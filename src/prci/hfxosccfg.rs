#[doc = "Register `hfxosccfg` reader"]
pub type R = crate::R<HfxosccfgSpec>;
#[doc = "Register `hfxosccfg` writer"]
pub type W = crate::W<HfxosccfgSpec>;
#[doc = "Field `enable` reader - "]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ready` reader - "]
pub type ReadyR = crate::BitReader;
#[doc = "Field `ready` writer - "]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<HfxosccfgSpec> {
        EnableW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> ReadyW<HfxosccfgSpec> {
        ReadyW::new(self, 31)
    }
}
#[doc = "Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxosccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxosccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxosccfgSpec;
impl crate::RegisterSpec for HfxosccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosccfg::R`](R) reader structure"]
impl crate::Readable for HfxosccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxosccfg::W`](W) writer structure"]
impl crate::Writable for HfxosccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hfxosccfg to value 0"]
impl crate::Resettable for HfxosccfgSpec {
    const RESET_VALUE: u32 = 0;
}
