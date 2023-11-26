#[doc = "Register `hfxosccfg` reader"]
pub type R = crate::R<HFXOSCCFG_SPEC>;
#[doc = "Register `hfxosccfg` writer"]
pub type W = crate::W<HFXOSCCFG_SPEC>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ready` reader - "]
pub type READY_R = crate::BitReader;
#[doc = "Field `ready` writer - "]
pub type READY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<HFXOSCCFG_SPEC> {
        ENABLE_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<HFXOSCCFG_SPEC> {
        READY_W::new(self, 31)
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
#[doc = "Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOSCCFG_SPEC;
impl crate::RegisterSpec for HFXOSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosccfg::R`](R) reader structure"]
impl crate::Readable for HFXOSCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxosccfg::W`](W) writer structure"]
impl crate::Writable for HFXOSCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hfxosccfg to value 0"]
impl crate::Resettable for HFXOSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
