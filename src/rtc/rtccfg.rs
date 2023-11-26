#[doc = "Register `rtccfg` reader"]
pub type R = crate::R<RTCCFG_SPEC>;
#[doc = "Register `rtccfg` writer"]
pub type W = crate::W<RTCCFG_SPEC>;
#[doc = "Field `scale` reader - "]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `scale` writer - "]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `enalways` reader - "]
pub type ENALWAYS_R = crate::BitReader;
#[doc = "Field `enalways` writer - "]
pub type ENALWAYS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
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
    pub fn scale(&mut self) -> SCALE_W<RTCCFG_SPEC> {
        SCALE_W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn enalways(&mut self) -> ENALWAYS_W<RTCCFG_SPEC> {
        ENALWAYS_W::new(self, 12)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cmpip(&mut self) -> CMPIP_W<RTCCFG_SPEC> {
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
#[doc = "RTC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCFG_SPEC;
impl crate::RegisterSpec for RTCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccfg::R`](R) reader structure"]
impl crate::Readable for RTCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccfg::W`](W) writer structure"]
impl crate::Writable for RTCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtccfg to value 0"]
impl crate::Resettable for RTCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
