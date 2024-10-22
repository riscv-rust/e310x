#[doc = "Register `rxdata` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Register `rxdata` writer"]
pub type W = crate::W<RxdataSpec>;
#[doc = "Field `data` reader - "]
pub type DataR = crate::FieldReader;
#[doc = "Field `data` writer - "]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `empty` reader - "]
pub type EmptyR = crate::BitReader;
#[doc = "Field `empty` writer - "]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<RxdataSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn empty(&mut self) -> EmptyW<RxdataSpec> {
        EmptyW::new(self, 31)
    }
}
#[doc = "Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdata::W`](W) writer structure"]
impl crate::Writable for RxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxdata to value 0"]
impl crate::Resettable for RxdataSpec {
    const RESET_VALUE: u32 = 0;
}
