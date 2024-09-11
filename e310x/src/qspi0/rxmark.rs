#[doc = "Register `rxmark` reader"]
pub type R = crate::R<RxmarkSpec>;
#[doc = "Register `rxmark` writer"]
pub type W = crate::W<RxmarkSpec>;
#[doc = "Field `rxmark` reader - Receive watermark"]
pub type RxmarkR = crate::FieldReader;
#[doc = "Field `rxmark` writer - Receive watermark"]
pub type RxmarkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    pub fn rxmark(&self) -> RxmarkR {
        RxmarkR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rxmark(&mut self) -> RxmarkW<RxmarkSpec> {
        RxmarkW::new(self, 0)
    }
}
#[doc = "Receive Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmark::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmark::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmarkSpec;
impl crate::RegisterSpec for RxmarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmark::R`](R) reader structure"]
impl crate::Readable for RxmarkSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmark::W`](W) writer structure"]
impl crate::Writable for RxmarkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxmark to value 0"]
impl crate::Resettable for RxmarkSpec {
    const RESET_VALUE: u32 = 0;
}
