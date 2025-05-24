#[doc = "Register `fctrl` reader"]
pub type R = crate::R<FctrlSpec>;
#[doc = "Register `fctrl` writer"]
pub type W = crate::W<FctrlSpec>;
#[doc = "Field `en` reader - SPI Flash Mode Select"]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - SPI Flash Mode Select"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Flash Mode Select"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Flash Mode Select"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<FctrlSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "SPI Flash Interface Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctrlSpec;
impl crate::RegisterSpec for FctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets fctrl to value 0"]
impl crate::Resettable for FctrlSpec {}
