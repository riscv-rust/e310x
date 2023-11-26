#[doc = "Register `txdata` reader"]
pub type R = crate::R<TXDATA_SPEC>;
#[doc = "Register `txdata` writer"]
pub type W = crate::W<TXDATA_SPEC>;
#[doc = "Field `data` reader - "]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `data` writer - "]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `full` reader - "]
pub type FULL_R = crate::BitReader;
#[doc = "Field `full` writer - "]
pub type FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TXDATA_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<TXDATA_SPEC> {
        FULL_W::new(self, 31)
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
#[doc = "Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TXDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txdata to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
