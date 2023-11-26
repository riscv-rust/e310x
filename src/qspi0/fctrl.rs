#[doc = "Register `fctrl` reader"]
pub type R = crate::R<FCTRL_SPEC>;
#[doc = "Register `fctrl` writer"]
pub type W = crate::W<FCTRL_SPEC>;
#[doc = "Field `en` reader - SPI Flash Mode Select"]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - SPI Flash Mode Select"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Flash Mode Select"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Flash Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<FCTRL_SPEC> {
        EN_W::new(self, 0)
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
#[doc = "SPI Flash Interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTRL_SPEC;
impl crate::RegisterSpec for FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fctrl to value 0"]
impl crate::Resettable for FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
