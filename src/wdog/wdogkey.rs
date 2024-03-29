#[doc = "Register `wdogkey` writer"]
pub struct W(crate::W<WDOGKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WDOGKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGKEY_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Key Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogkey](index.html) module"]
pub struct WDOGKEY_SPEC;
impl crate::RegisterSpec for WDOGKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdogkey::W](W) writer structure"]
impl crate::Writable for WDOGKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdogkey to value 0x0051_f15e"]
impl crate::Resettable for WDOGKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0051_f15e
    }
}
