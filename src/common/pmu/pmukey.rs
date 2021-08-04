#[doc = "Register `pmukey` reader"]
pub struct R(crate::R<PMUKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pmukey` writer"]
pub struct W(crate::W<PMUKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUKEY_SPEC>;
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
impl From<crate::W<PMUKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUKEY_SPEC>) -> Self {
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
#[doc = "PMU Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmukey](index.html) module"]
pub struct PMUKEY_SPEC;
impl crate::RegisterSpec for PMUKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmukey::R](R) reader structure"]
impl crate::Readable for PMUKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmukey::W](W) writer structure"]
impl crate::Writable for PMUKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pmukey to value 0"]
impl crate::Resettable for PMUKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
