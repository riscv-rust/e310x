#[doc = "Register `rxmark` reader"]
pub type R = crate::R<RXMARK_SPEC>;
#[doc = "Register `rxmark` writer"]
pub type W = crate::W<RXMARK_SPEC>;
#[doc = "Field `rxmark` reader - Receive watermark"]
pub type RXMARK_R = crate::FieldReader;
#[doc = "Field `rxmark` writer - Receive watermark"]
pub type RXMARK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    pub fn rxmark(&self) -> RXMARK_R {
        RXMARK_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rxmark(&mut self) -> RXMARK_W<RXMARK_SPEC> {
        RXMARK_W::new(self, 0)
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
#[doc = "Receive Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmark::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmark::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMARK_SPEC;
impl crate::RegisterSpec for RXMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmark::R`](R) reader structure"]
impl crate::Readable for RXMARK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxmark::W`](W) writer structure"]
impl crate::Writable for RXMARK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxmark to value 0"]
impl crate::Resettable for RXMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
