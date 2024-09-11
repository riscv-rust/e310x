#[doc = "Register `rtccfg` reader"]
pub type R = crate::R<RtccfgSpec>;
#[doc = "Register `rtccfg` writer"]
pub type W = crate::W<RtccfgSpec>;
#[doc = "Field `scale` reader - "]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `scale` writer - "]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `enalways` reader - "]
pub type EnalwaysR = crate::BitReader;
#[doc = "Field `enalways` writer - "]
pub type EnalwaysW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> EnalwaysR {
        EnalwaysR::new(((self.bits >> 12) & 1) != 0)
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
    pub fn scale(&mut self) -> ScaleW<RtccfgSpec> {
        ScaleW::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn enalways(&mut self) -> EnalwaysW<RtccfgSpec> {
        EnalwaysW::new(self, 12)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cmpip(&mut self) -> CmpipW<RtccfgSpec> {
        CmpipW::new(self, 28)
    }
}
#[doc = "RTC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccfgSpec;
impl crate::RegisterSpec for RtccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccfg::R`](R) reader structure"]
impl crate::Readable for RtccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccfg::W`](W) writer structure"]
impl crate::Writable for RtccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rtccfg to value 0"]
impl crate::Resettable for RtccfgSpec {
    const RESET_VALUE: u32 = 0;
}
