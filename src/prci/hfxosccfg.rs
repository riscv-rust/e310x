#[doc = "Register `hfxosccfg` reader"]
pub struct R(crate::R<HFXOSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hfxosccfg` writer"]
pub struct W(crate::W<HFXOSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSCCFG_SPEC>;
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
impl From<crate::W<HFXOSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFXOSCCFG_SPEC, bool, O>;
#[doc = "Field `ready` reader - "]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `ready` writer - "]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFXOSCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<30> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W<31> {
        READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosccfg](index.html) module"]
pub struct HFXOSCCFG_SPEC;
impl crate::RegisterSpec for HFXOSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxosccfg::R](R) reader structure"]
impl crate::Readable for HFXOSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxosccfg::W](W) writer structure"]
impl crate::Writable for HFXOSCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets hfxosccfg to value 0"]
impl crate::Resettable for HFXOSCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
