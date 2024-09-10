#[doc = "Register `txmark` reader"]
pub type R = crate::R<TxmarkSpec>;
#[doc = "Register `txmark` writer"]
pub type W = crate::W<TxmarkSpec>;
#[doc = "Field `txmark` reader - Transmit watermark"]
pub type TxmarkR = crate::FieldReader;
#[doc = "Field `txmark` writer - Transmit watermark"]
pub type TxmarkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    pub fn txmark(&self) -> TxmarkR {
        TxmarkR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txmark(&mut self) -> TxmarkW<TxmarkSpec> {
        TxmarkW::new(self, 0)
    }
}
#[doc = "Transmit Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txmark::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txmark::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxmarkSpec;
impl crate::RegisterSpec for TxmarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmark::R`](R) reader structure"]
impl crate::Readable for TxmarkSpec {}
#[doc = "`write(|w| ..)` method takes [`txmark::W`](W) writer structure"]
impl crate::Writable for TxmarkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txmark to value 0"]
impl crate::Resettable for TxmarkSpec {
    const RESET_VALUE: u32 = 0;
}
