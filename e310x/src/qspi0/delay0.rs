#[doc = "Register `delay0` reader"]
pub type R = crate::R<Delay0Spec>;
#[doc = "Register `delay0` writer"]
pub type W = crate::W<Delay0Spec>;
#[doc = "Field `cssck` reader - CS to SCK Delay"]
pub type CssckR = crate::FieldReader;
#[doc = "Field `cssck` writer - CS to SCK Delay"]
pub type CssckW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sckcs` reader - SCK to CS Delay"]
pub type SckcsR = crate::FieldReader;
#[doc = "Field `sckcs` writer - SCK to CS Delay"]
pub type SckcsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&self) -> CssckR {
        CssckR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&self) -> SckcsR {
        SckcsR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    #[must_use]
    pub fn cssck(&mut self) -> CssckW<Delay0Spec> {
        CssckW::new(self, 0)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sckcs(&mut self) -> SckcsW<Delay0Spec> {
        SckcsW::new(self, 16)
    }
}
#[doc = "Delay Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`delay0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Delay0Spec;
impl crate::RegisterSpec for Delay0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay0::R`](R) reader structure"]
impl crate::Readable for Delay0Spec {}
#[doc = "`write(|w| ..)` method takes [`delay0::W`](W) writer structure"]
impl crate::Writable for Delay0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets delay0 to value 0x0001_0001"]
impl crate::Resettable for Delay0Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
