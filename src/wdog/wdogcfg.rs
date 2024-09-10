#[doc = "Register `wdogcfg` reader"]
pub type R = crate::R<WdogcfgSpec>;
#[doc = "Register `wdogcfg` writer"]
pub type W = crate::W<WdogcfgSpec>;
#[doc = "Field `scale` reader - "]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `scale` writer - "]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rsten` reader - "]
pub type RstenR = crate::BitReader;
#[doc = "Field `rsten` writer - "]
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `zerocmp` reader - "]
pub type ZerocmpR = crate::BitReader;
#[doc = "Field `zerocmp` writer - "]
pub type ZerocmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enalways` reader - "]
pub type EnalwaysR = crate::BitReader;
#[doc = "Field `enalways` writer - "]
pub type EnalwaysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `encoreawake` reader - "]
pub type EncoreawakeR = crate::BitReader;
#[doc = "Field `encoreawake` writer - "]
pub type EncoreawakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmpip` reader - "]
pub type CmpipR = crate::BitReader;
#[doc = "Field `cmpip` writer - "]
pub type CmpipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsten(&self) -> RstenR {
        RstenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZerocmpR {
        ZerocmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> EnalwaysR {
        EnalwaysR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn encoreawake(&self) -> EncoreawakeR {
        EncoreawakeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&self) -> CmpipR {
        CmpipR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<WdogcfgSpec> {
        ScaleW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RstenW<WdogcfgSpec> {
        RstenW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn zerocmp(&mut self) -> ZerocmpW<WdogcfgSpec> {
        ZerocmpW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn enalways(&mut self) -> EnalwaysW<WdogcfgSpec> {
        EnalwaysW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn encoreawake(&mut self) -> EncoreawakeW<WdogcfgSpec> {
        EncoreawakeW::new(self, 13)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cmpip(&mut self) -> CmpipW<WdogcfgSpec> {
        CmpipW::new(self, 28)
    }
}
#[doc = "Watchdog Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogcfgSpec;
impl crate::RegisterSpec for WdogcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcfg::R`](R) reader structure"]
impl crate::Readable for WdogcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogcfg::W`](W) writer structure"]
impl crate::Writable for WdogcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdogcfg to value 0"]
impl crate::Resettable for WdogcfgSpec {
    const RESET_VALUE: u32 = 0;
}
