#[doc = "Register `pmuie` reader"]
pub type R = crate::R<PmuieSpec>;
#[doc = "Register `pmuie` writer"]
pub type W = crate::W<PmuieSpec>;
#[doc = "Field `rtc` reader - "]
pub type RtcR = crate::BitReader;
#[doc = "Field `rtc` writer - "]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dwakeup` reader - "]
pub type DwakeupR = crate::BitReader;
#[doc = "Field `dwakeup` writer - "]
pub type DwakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `awakeup` reader - "]
pub type AwakeupR = crate::BitReader;
#[doc = "Field `awakeup` writer - "]
pub type AwakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&self) -> DwakeupR {
        DwakeupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&self) -> AwakeupR {
        AwakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<PmuieSpec> {
        RtcW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dwakeup(&mut self) -> DwakeupW<PmuieSpec> {
        DwakeupW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn awakeup(&mut self) -> AwakeupW<PmuieSpec> {
        AwakeupW::new(self, 3)
    }
}
#[doc = "PMU Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuieSpec;
impl crate::RegisterSpec for PmuieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmuie::R`](R) reader structure"]
impl crate::Readable for PmuieSpec {}
#[doc = "`write(|w| ..)` method takes [`pmuie::W`](W) writer structure"]
impl crate::Writable for PmuieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pmuie to value 0"]
impl crate::Resettable for PmuieSpec {
    const RESET_VALUE: u32 = 0;
}
