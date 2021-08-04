#[doc = "Register `csdef` reader"]
pub struct R(crate::R<CSDEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSDEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSDEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSDEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csdef` writer"]
pub struct W(crate::W<CSDEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSDEF_SPEC>;
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
impl From<crate::W<CSDEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSDEF_SPEC>) -> Self {
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
#[doc = "Chip Select Default Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdef](index.html) module"]
pub struct CSDEF_SPEC;
impl crate::RegisterSpec for CSDEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csdef::R](R) reader structure"]
impl crate::Readable for CSDEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csdef::W](W) writer structure"]
impl crate::Writable for CSDEF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csdef to value 0xffff"]
impl crate::Resettable for CSDEF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
