#[doc = "Register `pwms` reader"]
pub struct R(crate::R<PWMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwms` writer"]
pub struct W(crate::W<PWMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMS_SPEC>;
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
impl From<crate::W<PWMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMS_SPEC>) -> Self {
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
#[doc = "Scaled Halfword Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwms](index.html) module"]
pub struct PWMS_SPEC;
impl crate::RegisterSpec for PWMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwms::R](R) reader structure"]
impl crate::Readable for PWMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwms::W](W) writer structure"]
impl crate::Writable for PWMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pwms to value 0"]
impl crate::Resettable for PWMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
