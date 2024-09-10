#[doc = "Register `ctr` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `ctr` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `ien` reader - I2C core interrupt enable bit"]
pub type IenR = crate::BitReader;
#[doc = "Field `ien` writer - I2C core interrupt enable bit"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en` reader - I2C core enable bit"]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - I2C core enable bit"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - I2C core interrupt enable bit"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C core enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - I2C core interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IenW<CtrSpec> {
        IenW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C core enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrSpec> {
        EnW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctr to value 0"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0;
}
