#[doc = "Register `claim` reader"]
pub struct R(crate::R<CLAIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `claim` writer"]
pub struct W(crate::W<CLAIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIM_SPEC>;
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
impl From<crate::W<CLAIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIM_SPEC>) -> Self {
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
#[doc = "Claim/Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claim](index.html) module"]
pub struct CLAIM_SPEC;
impl crate::RegisterSpec for CLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claim::R](R) reader structure"]
impl crate::Readable for CLAIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claim::W](W) writer structure"]
impl crate::Writable for CLAIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets claim to value 0"]
impl crate::Resettable for CLAIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
