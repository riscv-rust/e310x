#[doc = "Register `txmark` reader"]
pub type R = crate::R<TXMARK_SPEC>;
#[doc = "Register `txmark` writer"]
pub type W = crate::W<TXMARK_SPEC>;
#[doc = "Field `txmark` reader - Transmit watermark"]
pub type TXMARK_R = crate::FieldReader;
#[doc = "Field `txmark` writer - Transmit watermark"]
pub type TXMARK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    pub fn txmark(&self) -> TXMARK_R {
        TXMARK_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txmark(&mut self) -> TXMARK_W<TXMARK_SPEC> {
        TXMARK_W::new(self, 0)
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
#[doc = "Transmit Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmark::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmark::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMARK_SPEC;
impl crate::RegisterSpec for TXMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmark::R`](R) reader structure"]
impl crate::Readable for TXMARK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmark::W`](W) writer structure"]
impl crate::Writable for TXMARK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txmark to value 0"]
impl crate::Resettable for TXMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
