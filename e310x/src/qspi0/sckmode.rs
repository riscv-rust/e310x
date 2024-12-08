#[doc = "Register `sckmode` reader"]
pub type R = crate::R<SckmodeSpec>;
#[doc = "Register `sckmode` writer"]
pub type W = crate::W<SckmodeSpec>;
#[doc = "Field `pha` reader - Serial clock phase"]
pub type PhaR = crate::BitReader;
#[doc = "Field `pha` writer - Serial clock phase"]
pub type PhaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pol` reader - Serial clock polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `pol` writer - Serial clock polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&self) -> PhaR {
        PhaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&mut self) -> PhaW<SckmodeSpec> {
        PhaW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<SckmodeSpec> {
        PolW::new(self, 1)
    }
}
#[doc = "Serial Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckmodeSpec;
impl crate::RegisterSpec for SckmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sckmode::R`](R) reader structure"]
impl crate::Readable for SckmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`sckmode::W`](W) writer structure"]
impl crate::Writable for SckmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sckmode to value 0"]
impl crate::Resettable for SckmodeSpec {
    const RESET_VALUE: u32 = 0;
}
