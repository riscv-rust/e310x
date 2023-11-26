#[doc = "Register `ip` reader"]
pub type R = crate::R<IP_SPEC>;
#[doc = "Register `ip` writer"]
pub type W = crate::W<IP_SPEC>;
#[doc = "Field `txwm` reader - Transmit watermark enable"]
pub type TXWM_R = crate::BitReader;
#[doc = "Field `txwm` writer - Transmit watermark enable"]
pub type TXWM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxwm` reader - Receive watermark enable"]
pub type RXWM_R = crate::BitReader;
#[doc = "Field `rxwm` writer - Receive watermark enable"]
pub type RXWM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit watermark enable"]
    #[inline(always)]
    pub fn txwm(&self) -> TXWM_R {
        TXWM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive watermark enable"]
    #[inline(always)]
    pub fn rxwm(&self) -> RXWM_R {
        RXWM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit watermark enable"]
    #[inline(always)]
    #[must_use]
    pub fn txwm(&mut self) -> TXWM_W<IP_SPEC> {
        TXWM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive watermark enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxwm(&mut self) -> RXWM_W<IP_SPEC> {
        RXWM_W::new(self, 1)
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
#[doc = "SPI Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IP_SPEC;
impl crate::RegisterSpec for IP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip::R`](R) reader structure"]
impl crate::Readable for IP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ip::W`](W) writer structure"]
impl crate::Writable for IP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ip to value 0"]
impl crate::Resettable for IP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
