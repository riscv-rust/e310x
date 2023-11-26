#[doc = "Register `wdogcmp` reader"]
pub type R = crate::R<WDOGCMP_SPEC>;
#[doc = "Register `wdogcmp` writer"]
pub type W = crate::W<WDOGCMP_SPEC>;
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<WDOGCMP_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "Watchdog Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGCMP_SPEC;
impl crate::RegisterSpec for WDOGCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcmp::R`](R) reader structure"]
impl crate::Readable for WDOGCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdogcmp::W`](W) writer structure"]
impl crate::Writable for WDOGCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdogcmp to value 0"]
impl crate::Resettable for WDOGCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
