#[doc = "Register `plloutdiv` reader"]
pub type R = crate::R<PlloutdivSpec>;
#[doc = "Register `plloutdiv` writer"]
pub type W = crate::W<PlloutdivSpec>;
#[doc = "Field `div` reader - "]
pub type DivR = crate::FieldReader;
#[doc = "Field `div` writer - "]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `divby1` reader - "]
pub type Divby1R = crate::BitReader;
#[doc = "Field `divby1` writer - "]
pub type Divby1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&self) -> Divby1R {
        Divby1R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<PlloutdivSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&mut self) -> Divby1W<PlloutdivSpec> {
        Divby1W::new(self, 8)
    }
}
#[doc = "PLL Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plloutdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plloutdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlloutdivSpec;
impl crate::RegisterSpec for PlloutdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plloutdiv::R`](R) reader structure"]
impl crate::Readable for PlloutdivSpec {}
#[doc = "`write(|w| ..)` method takes [`plloutdiv::W`](W) writer structure"]
impl crate::Writable for PlloutdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets plloutdiv to value 0x0100"]
impl crate::Resettable for PlloutdivSpec {
    const RESET_VALUE: u32 = 0x0100;
}
