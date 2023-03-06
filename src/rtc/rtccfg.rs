#[doc = "Register `rtccfg` reader"]
pub struct R(crate::R<RTCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtccfg` writer"]
pub struct W(crate::W<RTCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCFG_SPEC>;
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
impl From<crate::W<RTCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `scale` reader - "]
pub type SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `scale` writer - "]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `enalways` reader - "]
pub type ENALWAYS_R = crate::BitReader<bool>;
#[doc = "Field `enalways` writer - "]
pub type ENALWAYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCFG_SPEC, bool, O>;
#[doc = "Field `cmpip` reader - "]
pub type CMPIP_R = crate::BitReader<bool>;
#[doc = "Field `cmpip` writer - "]
pub type CMPIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&self) -> CMPIP_R {
        CMPIP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<0> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W<12> {
        ENALWAYS_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&mut self) -> CMPIP_W<28> {
        CMPIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccfg](index.html) module"]
pub struct RTCCFG_SPEC;
impl crate::RegisterSpec for RTCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccfg::R](R) reader structure"]
impl crate::Readable for RTCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccfg::W](W) writer structure"]
impl crate::Writable for RTCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rtccfg to value 0"]
impl crate::Resettable for RTCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
