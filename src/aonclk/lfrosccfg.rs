#[doc = "Register `lfrosccfg` reader"]
pub type R = crate::R<LFROSCCFG_SPEC>;
#[doc = "Register `lfrosccfg` writer"]
pub type W = crate::W<LFROSCCFG_SPEC>;
#[doc = "Field `div` reader - "]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `div` writer - "]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `trim` reader - "]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `trim` writer - "]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ready` reader - "]
pub type READY_R = crate::BitReader;
#[doc = "Field `ready` writer - "]
pub type READY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
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
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<LFROSCCFG_SPEC> {
        DIV_W::new(self, 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<LFROSCCFG_SPEC> {
        TRIM_W::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<LFROSCCFG_SPEC> {
        ENABLE_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<LFROSCCFG_SPEC> {
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
#[doc = "AON Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrosccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrosccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFROSCCFG_SPEC;
impl crate::RegisterSpec for LFROSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrosccfg::R`](R) reader structure"]
impl crate::Readable for LFROSCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfrosccfg::W`](W) writer structure"]
impl crate::Writable for LFROSCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lfrosccfg to value 0"]
impl crate::Resettable for LFROSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
