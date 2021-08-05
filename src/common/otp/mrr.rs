#[doc = "Register `mrr` reader"]
pub struct R(crate::R<MRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mrr` writer"]
pub struct W(crate::W<MRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRR_SPEC>;
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
impl From<crate::W<MRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRR_SPEC>) -> Self {
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
#[doc = "OTP read-voltage regulator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrr](index.html) module"]
pub struct MRR_SPEC;
impl crate::RegisterSpec for MRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrr::R](R) reader structure"]
impl crate::Readable for MRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrr::W](W) writer structure"]
impl crate::Writable for MRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mrr to value 0"]
impl crate::Resettable for MRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
