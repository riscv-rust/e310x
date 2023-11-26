#[doc = "Register `rxdata` reader"]
pub type R = crate::R<RXDATA_SPEC>;
#[doc = "Register `rxdata` writer"]
pub type W = crate::W<RXDATA_SPEC>;
#[doc = "Field `data` reader - Received data"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `data` writer - Received data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `empty` reader - FIFO empty flag"]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `empty` writer - FIFO empty flag"]
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Received data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - FIFO empty flag"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Received data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<RXDATA_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bit 31 - FIFO empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<RXDATA_SPEC> {
        EMPTY_W::new(self, 31)
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
#[doc = "Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdata::W`](W) writer structure"]
impl crate::Writable for RXDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdata to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
