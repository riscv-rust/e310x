#[doc = "Register `coreclkcfg` reader"]
pub struct R(crate::R<CORECLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORECLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORECLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORECLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `coreclkcfg` writer"]
pub struct W(crate::W<CORECLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORECLKCFG_SPEC>;
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
impl From<crate::W<CORECLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORECLKCFG_SPEC>) -> Self {
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
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coreclkcfg](index.html) module"]
pub struct CORECLKCFG_SPEC;
impl crate::RegisterSpec for CORECLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coreclkcfg::R](R) reader structure"]
impl crate::Readable for CORECLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coreclkcfg::W](W) writer structure"]
impl crate::Writable for CORECLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets coreclkcfg to value 0"]
impl crate::Resettable for CORECLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
