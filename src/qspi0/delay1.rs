#[doc = "Register `delay1` reader"]
pub type R = crate::R<DELAY1_SPEC>;
#[doc = "Register `delay1` writer"]
pub type W = crate::W<DELAY1_SPEC>;
#[doc = "Field `intercs` reader - Minimum CS inactive time"]
pub type INTERCS_R = crate::FieldReader;
#[doc = "Field `intercs` writer - Minimum CS inactive time"]
pub type INTERCS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `interxfr` reader - Maximum interframe delay"]
pub type INTERXFR_R = crate::FieldReader;
#[doc = "Field `interxfr` writer - Maximum interframe delay"]
pub type INTERXFR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&self) -> INTERCS_R {
        INTERCS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&self) -> INTERXFR_R {
        INTERXFR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    #[must_use]
    pub fn intercs(&mut self) -> INTERCS_W<DELAY1_SPEC> {
        INTERCS_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    #[must_use]
    pub fn interxfr(&mut self) -> INTERXFR_W<DELAY1_SPEC> {
        INTERXFR_W::new(self, 16)
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
#[doc = "Delay Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DELAY1_SPEC;
impl crate::RegisterSpec for DELAY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay1::R`](R) reader structure"]
impl crate::Readable for DELAY1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`delay1::W`](W) writer structure"]
impl crate::Writable for DELAY1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets delay1 to value 0x01"]
impl crate::Resettable for DELAY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
