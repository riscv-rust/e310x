#[doc = "Register `div` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `div` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl From<crate::W<DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `value` reader - "]
pub struct VALUE_R(crate::FieldReader<u16, u16>);
impl VALUE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `value` writer - "]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets div to value 0"]
impl crate::Resettable for DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
