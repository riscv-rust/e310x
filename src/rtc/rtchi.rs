#[doc = "Register `rtchi` reader"]
pub struct R(crate::R<RTCHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtchi` writer"]
pub struct W(crate::W<RTCHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCHI_SPEC>;
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
impl From<crate::W<RTCHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCHI_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Counter High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtchi](index.html) module"]
pub struct RTCHI_SPEC;
impl crate::RegisterSpec for RTCHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtchi::R](R) reader structure"]
impl crate::Readable for RTCHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtchi::W](W) writer structure"]
impl crate::Writable for RTCHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rtchi to value 0"]
impl crate::Resettable for RTCHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
