#[doc = "Register `delay0` reader"]
pub type R = crate::R<DELAY0_SPEC>;
#[doc = "Register `delay0` writer"]
pub type W = crate::W<DELAY0_SPEC>;
#[doc = "Field `cssck` reader - CS to SCK Delay"]
pub type CSSCK_R = crate::FieldReader;
#[doc = "Field `cssck` writer - CS to SCK Delay"]
pub type CSSCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sckcs` reader - SCK to CS Delay"]
pub type SCKCS_R = crate::FieldReader;
#[doc = "Field `sckcs` writer - SCK to CS Delay"]
pub type SCKCS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&self) -> SCKCS_R {
        SCKCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    #[must_use]
    pub fn cssck(&mut self) -> CSSCK_W<DELAY0_SPEC> {
        CSSCK_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sckcs(&mut self) -> SCKCS_W<DELAY0_SPEC> {
        SCKCS_W::new(self, 16)
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
#[doc = "Delay Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DELAY0_SPEC;
impl crate::RegisterSpec for DELAY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay0::R`](R) reader structure"]
impl crate::Readable for DELAY0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`delay0::W`](W) writer structure"]
impl crate::Writable for DELAY0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets delay0 to value 0x0001_0001"]
impl crate::Resettable for DELAY0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
