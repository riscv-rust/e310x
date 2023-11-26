#[doc = "Register `rxctrl` reader"]
pub type R = crate::R<RXCTRL_SPEC>;
#[doc = "Register `rxctrl` writer"]
pub type W = crate::W<RXCTRL_SPEC>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `counter` reader - "]
pub type COUNTER_R = crate::FieldReader;
#[doc = "Field `counter` writer - "]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<RXCTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<RXCTRL_SPEC> {
        COUNTER_W::new(self, 16)
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
#[doc = "Receive Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCTRL_SPEC;
impl crate::RegisterSpec for RXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl::R`](R) reader structure"]
impl crate::Readable for RXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxctrl::W`](W) writer structure"]
impl crate::Writable for RXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxctrl to value 0"]
impl crate::Resettable for RXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
