#[doc = "Register `cfg` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `scale` reader - "]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `scale` writer - "]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sticky` reader - "]
pub type StickyR = crate::BitReader;
#[doc = "Field `sticky` writer - "]
pub type StickyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `zerocmp` reader - "]
pub type ZerocmpR = crate::BitReader;
#[doc = "Field `zerocmp` writer - "]
pub type ZerocmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `deglitch` reader - "]
pub type DeglitchR = crate::BitReader;
#[doc = "Field `deglitch` writer - "]
pub type DeglitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enalways` reader - "]
pub type EnalwaysR = crate::BitReader;
#[doc = "Field `enalways` writer - "]
pub type EnalwaysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enoneshot` reader - "]
pub type EnoneshotR = crate::BitReader;
#[doc = "Field `enoneshot` writer - "]
pub type EnoneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp0center` reader - "]
pub type Cmp0centerR = crate::BitReader;
#[doc = "Field `cmp0center` writer - "]
pub type Cmp0centerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp1center` reader - "]
pub type Cmp1centerR = crate::BitReader;
#[doc = "Field `cmp1center` writer - "]
pub type Cmp1centerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp2center` reader - "]
pub type Cmp2centerR = crate::BitReader;
#[doc = "Field `cmp2center` writer - "]
pub type Cmp2centerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp3center` reader - "]
pub type Cmp3centerR = crate::BitReader;
#[doc = "Field `cmp3center` writer - "]
pub type Cmp3centerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp0gang` reader - "]
pub type Cmp0gangR = crate::BitReader;
#[doc = "Field `cmp0gang` writer - "]
pub type Cmp0gangW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp1gang` reader - "]
pub type Cmp1gangR = crate::BitReader;
#[doc = "Field `cmp1gang` writer - "]
pub type Cmp1gangW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp2gang` reader - "]
pub type Cmp2gangR = crate::FieldReader<u16>;
#[doc = "Field `cmp2gang` writer - "]
pub type Cmp2gangW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `cmp3gang` reader - "]
pub type Cmp3gangR = crate::BitReader;
#[doc = "Field `cmp3gang` writer - "]
pub type Cmp3gangW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp0ip` reader - "]
pub type Cmp0ipR = crate::BitReader;
#[doc = "Field `cmp0ip` writer - "]
pub type Cmp0ipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp1ip` reader - "]
pub type Cmp1ipR = crate::BitReader;
#[doc = "Field `cmp1ip` writer - "]
pub type Cmp1ipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp2ip` reader - "]
pub type Cmp2ipR = crate::BitReader;
#[doc = "Field `cmp2ip` writer - "]
pub type Cmp2ipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp3ip` reader - "]
pub type Cmp3ipR = crate::BitReader;
#[doc = "Field `cmp3ip` writer - "]
pub type Cmp3ipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&self) -> StickyR {
        StickyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZerocmpR {
        ZerocmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&self) -> DeglitchR {
        DeglitchR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> EnalwaysR {
        EnalwaysR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&self) -> EnoneshotR {
        EnoneshotR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&self) -> Cmp0centerR {
        Cmp0centerR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&self) -> Cmp1centerR {
        Cmp1centerR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&self) -> Cmp2centerR {
        Cmp2centerR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&self) -> Cmp3centerR {
        Cmp3centerR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&self) -> Cmp0gangR {
        Cmp0gangR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&self) -> Cmp1gangR {
        Cmp1gangR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&self) -> Cmp2gangR {
        Cmp2gangR::new(((self.bits >> 26) & 0x07ff) as u16)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&self) -> Cmp3gangR {
        Cmp3gangR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&self) -> Cmp0ipR {
        Cmp0ipR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&self) -> Cmp1ipR {
        Cmp1ipR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&self) -> Cmp2ipR {
        Cmp2ipR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&self) -> Cmp3ipR {
        Cmp3ipR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<CfgSpec> {
        ScaleW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sticky(&mut self) -> StickyW<CfgSpec> {
        StickyW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn zerocmp(&mut self) -> ZerocmpW<CfgSpec> {
        ZerocmpW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn deglitch(&mut self) -> DeglitchW<CfgSpec> {
        DeglitchW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn enalways(&mut self) -> EnalwaysW<CfgSpec> {
        EnalwaysW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn enoneshot(&mut self) -> EnoneshotW<CfgSpec> {
        EnoneshotW::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0center(&mut self) -> Cmp0centerW<CfgSpec> {
        Cmp0centerW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1center(&mut self) -> Cmp1centerW<CfgSpec> {
        Cmp1centerW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2center(&mut self) -> Cmp2centerW<CfgSpec> {
        Cmp2centerW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3center(&mut self) -> Cmp3centerW<CfgSpec> {
        Cmp3centerW::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0gang(&mut self) -> Cmp0gangW<CfgSpec> {
        Cmp0gangW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1gang(&mut self) -> Cmp1gangW<CfgSpec> {
        Cmp1gangW::new(self, 25)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2gang(&mut self) -> Cmp2gangW<CfgSpec> {
        Cmp2gangW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3gang(&mut self) -> Cmp3gangW<CfgSpec> {
        Cmp3gangW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ip(&mut self) -> Cmp0ipW<CfgSpec> {
        Cmp0ipW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ip(&mut self) -> Cmp1ipW<CfgSpec> {
        Cmp1ipW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ip(&mut self) -> Cmp2ipW<CfgSpec> {
        Cmp2ipW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ip(&mut self) -> Cmp3ipW<CfgSpec> {
        Cmp3ipW::new(self, 31)
    }
}
#[doc = "PWM Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
