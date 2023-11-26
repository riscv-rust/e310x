#[doc = "Register `sckdiv` reader"]
pub type R = crate::R<SCKDIV_SPEC>;
#[doc = "Register `sckdiv` writer"]
pub type W = crate::W<SCKDIV_SPEC>;
#[doc = "Field `div` reader - Divisor for serial clock"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `div` writer - Divisor for serial clock"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Divisor for serial clock"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Divisor for serial clock"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<SCKDIV_SPEC> {
        DIV_W::new(self, 0)
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
#[doc = "Serial Clock Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCKDIV_SPEC;
impl crate::RegisterSpec for SCKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sckdiv::R`](R) reader structure"]
impl crate::Readable for SCKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sckdiv::W`](W) writer structure"]
impl crate::Writable for SCKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sckdiv to value 0"]
impl crate::Resettable for SCKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
