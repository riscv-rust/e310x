#[doc = "Register `write_en` reader"]
pub struct R(crate::R<WRITE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `write_en` writer"]
pub struct W(crate::W<WRITE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITE_EN_SPEC>;
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
impl From<crate::W<WRITE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITE_EN_SPEC>) -> Self {
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
#[doc = "OTP device write-enable signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [write_en](index.html) module"]
pub struct WRITE_EN_SPEC;
impl crate::RegisterSpec for WRITE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [write_en::R](R) reader structure"]
impl crate::Readable for WRITE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [write_en::W](W) writer structure"]
impl crate::Writable for WRITE_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets write_en to value 0"]
impl crate::Resettable for WRITE_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
