#[doc = "Register `ctr` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `ctr` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `ien` reader - I2C core interrupt enable bit"]
pub type IEN_R = crate::BitReader;
#[doc = "Field `ien` writer - I2C core interrupt enable bit"]
pub type IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en` reader - I2C core enable bit"]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - I2C core enable bit"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - I2C core interrupt enable bit"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C core enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - I2C core interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<CTR_SPEC> {
        IEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C core enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTR_SPEC> {
        EN_W::new(self, 7)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ctr to value 0"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
