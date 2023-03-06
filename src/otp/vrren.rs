#[doc = "Register `vrren` reader"]
pub struct R(crate::R<VRREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VRREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VRREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VRREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vrren` writer"]
pub struct W(crate::W<VRREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VRREN_SPEC>;
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
impl From<crate::W<VRREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VRREN_SPEC>) -> Self {
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
#[doc = "OTP read-voltage enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrren](index.html) module"]
pub struct VRREN_SPEC;
impl crate::RegisterSpec for VRREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrren::R](R) reader structure"]
impl crate::Readable for VRREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrren::W](W) writer structure"]
impl crate::Writable for VRREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vrren to value 0"]
impl crate::Resettable for VRREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
