#[doc = "Register `delay0` reader"]
pub struct R(crate::R<DELAY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `delay0` writer"]
pub struct W(crate::W<DELAY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY0_SPEC>;
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
impl From<crate::W<DELAY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cssck` reader - CS to SCK Delay"]
pub type CSSCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cssck` writer - CS to SCK Delay"]
pub type CSSCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY0_SPEC, u8, u8, 8, O>;
#[doc = "Field `sckcs` reader - SCK to CS Delay"]
pub type SCKCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sckcs` writer - SCK to CS Delay"]
pub type SCKCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&self) -> SCKCS_R {
        SCKCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&mut self) -> CSSCK_W<0> {
        CSSCK_W::new(self)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&mut self) -> SCKCS_W<16> {
        SCKCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay0](index.html) module"]
pub struct DELAY0_SPEC;
impl crate::RegisterSpec for DELAY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay0::R](R) reader structure"]
impl crate::Readable for DELAY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay0::W](W) writer structure"]
impl crate::Writable for DELAY0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets delay0 to value 0x0001_0001"]
impl crate::Resettable for DELAY0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}
