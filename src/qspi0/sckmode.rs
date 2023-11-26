#[doc = "Register `sckmode` reader"]
pub type R = crate::R<SCKMODE_SPEC>;
#[doc = "Register `sckmode` writer"]
pub type W = crate::W<SCKMODE_SPEC>;
#[doc = "Field `pha` reader - Serial clock phase"]
pub type PHA_R = crate::BitReader;
#[doc = "Field `pha` writer - Serial clock phase"]
pub type PHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pol` reader - Serial clock polarity"]
pub type POL_R = crate::BitReader;
#[doc = "Field `pol` writer - Serial clock polarity"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn pha(&mut self) -> PHA_W<SCKMODE_SPEC> {
        PHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<SCKMODE_SPEC> {
        POL_W::new(self, 1)
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
#[doc = "Serial Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCKMODE_SPEC;
impl crate::RegisterSpec for SCKMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sckmode::R`](R) reader structure"]
impl crate::Readable for SCKMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sckmode::W`](W) writer structure"]
impl crate::Writable for SCKMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sckmode to value 0"]
impl crate::Resettable for SCKMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
