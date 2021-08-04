#[doc = "Register `rtccmp` reader"]
pub struct R(crate::R<RTCCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtccmp` writer"]
pub struct W(crate::W<RTCCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCMP_SPEC>;
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
impl From<crate::W<RTCCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCMP_SPEC>) -> Self {
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
#[doc = "RTC Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccmp](index.html) module"]
pub struct RTCCMP_SPEC;
impl crate::RegisterSpec for RTCCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccmp::R](R) reader structure"]
impl crate::Readable for RTCCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccmp::W](W) writer structure"]
impl crate::Writable for RTCCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rtccmp to value 0"]
impl crate::Resettable for RTCCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
