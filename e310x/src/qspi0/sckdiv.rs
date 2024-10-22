#[doc = "Register `sckdiv` reader"]
pub type R = crate::R<SckdivSpec>;
#[doc = "Register `sckdiv` writer"]
pub type W = crate::W<SckdivSpec>;
#[doc = "Field `div` reader - Divisor for serial clock"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `div` writer - Divisor for serial clock"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Divisor for serial clock"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Divisor for serial clock"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<SckdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Serial Clock Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckdivSpec;
impl crate::RegisterSpec for SckdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sckdiv::R`](R) reader structure"]
impl crate::Readable for SckdivSpec {}
#[doc = "`write(|w| ..)` method takes [`sckdiv::W`](W) writer structure"]
impl crate::Writable for SckdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sckdiv to value 0"]
impl crate::Resettable for SckdivSpec {
    const RESET_VALUE: u32 = 0;
}
