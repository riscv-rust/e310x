#[doc = "Register `rxdata` reader"]
pub struct R(crate::R<RXDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxdata` writer"]
pub struct W(crate::W<RXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDATA_SPEC>;
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
impl From<crate::W<RXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data` reader - "]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `data` writer - "]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXDATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `empty` reader - "]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `empty` writer - "]
pub type EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXDATA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W<31> {
        EMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](index.html) module"]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdata::R](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdata::W](W) writer structure"]
impl crate::Writable for RXDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rxdata to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
