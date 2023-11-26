#[doc = "Register `wdogcfg` reader"]
pub type R = crate::R<WDOGCFG_SPEC>;
#[doc = "Register `wdogcfg` writer"]
pub type W = crate::W<WDOGCFG_SPEC>;
#[doc = "Field `scale` reader - "]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `scale` writer - "]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rsten` reader - "]
pub type RSTEN_R = crate::BitReader;
#[doc = "Field `rsten` writer - "]
pub type RSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `zerocmp` reader - "]
pub type ZEROCMP_R = crate::BitReader;
#[doc = "Field `zerocmp` writer - "]
pub type ZEROCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enalways` reader - "]
pub type ENALWAYS_R = crate::BitReader;
#[doc = "Field `enalways` writer - "]
pub type ENALWAYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `encoreawake` reader - "]
pub type ENCOREAWAKE_R = crate::BitReader;
#[doc = "Field `encoreawake` writer - "]
pub type ENCOREAWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmpip` reader - "]
pub type CMPIP_R = crate::BitReader;
#[doc = "Field `cmpip` writer - "]
pub type CMPIP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZEROCMP_R {
        ZEROCMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn encoreawake(&self) -> ENCOREAWAKE_R {
        ENCOREAWAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&self) -> CMPIP_R {
        CMPIP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<WDOGCFG_SPEC> {
        SCALE_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<WDOGCFG_SPEC> {
        RSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn zerocmp(&mut self) -> ZEROCMP_W<WDOGCFG_SPEC> {
        ZEROCMP_W::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn enalways(&mut self) -> ENALWAYS_W<WDOGCFG_SPEC> {
        ENALWAYS_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn encoreawake(&mut self) -> ENCOREAWAKE_W<WDOGCFG_SPEC> {
        ENCOREAWAKE_W::new(self, 13)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cmpip(&mut self) -> CMPIP_W<WDOGCFG_SPEC> {
        CMPIP_W::new(self, 28)
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
#[doc = "Watchdog Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGCFG_SPEC;
impl crate::RegisterSpec for WDOGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcfg::R`](R) reader structure"]
impl crate::Readable for WDOGCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdogcfg::W`](W) writer structure"]
impl crate::Writable for WDOGCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdogcfg to value 0"]
impl crate::Resettable for WDOGCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
