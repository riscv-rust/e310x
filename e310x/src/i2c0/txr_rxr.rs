#[doc = "Register `txr_rxr` reader"]
pub type R = crate::R<TxrRxrSpec>;
#[doc = "Register `txr_rxr` writer"]
pub type W = crate::W<TxrRxrSpec>;
#[doc = "Field `data` reader - "]
pub type DataR = crate::FieldReader;
#[doc = "Field `data` writer - "]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TxrRxrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Transmit register / Receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`txr_rxr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txr_rxr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxrRxrSpec;
impl crate::RegisterSpec for TxrRxrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txr_rxr::R`](R) reader structure"]
impl crate::Readable for TxrRxrSpec {}
#[doc = "`write(|w| ..)` method takes [`txr_rxr::W`](W) writer structure"]
impl crate::Writable for TxrRxrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txr_rxr to value 0"]
impl crate::Resettable for TxrRxrSpec {
    const RESET_VALUE: u32 = 0;
}
