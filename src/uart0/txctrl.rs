#[doc = "Register `txctrl` reader"]
pub type R = crate::R<TXCTRL_SPEC>;
#[doc = "Register `txctrl` writer"]
pub type W = crate::W<TXCTRL_SPEC>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nstop` reader - "]
pub type NSTOP_R = crate::BitReader;
#[doc = "Field `nstop` writer - "]
pub type NSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&self) -> NSTOP_R {
        NSTOP_R::new(((self.bits >> 1) & 1) != 0)
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
    pub fn enable(&mut self) -> ENABLE_W<TXCTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn nstop(&mut self) -> NSTOP_W<TXCTRL_SPEC> {
        NSTOP_W::new(self, 1)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<TXCTRL_SPEC> {
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
#[doc = "Transmit Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txctrl to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
