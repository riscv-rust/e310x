#[doc = "Register `wdogcount` reader"]
pub struct R(crate::R<WDOGCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdogcount` writer"]
pub struct W(crate::W<WDOGCOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGCOUNT_SPEC>;
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
impl From<crate::W<WDOGCOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGCOUNT_SPEC>) -> Self {
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
#[doc = "Watchdog Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogcount](index.html) module"]
pub struct WDOGCOUNT_SPEC;
impl crate::RegisterSpec for WDOGCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogcount::R](R) reader structure"]
impl crate::Readable for WDOGCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogcount::W](W) writer structure"]
impl crate::Writable for WDOGCOUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdogcount to value 0"]
impl crate::Resettable for WDOGCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
