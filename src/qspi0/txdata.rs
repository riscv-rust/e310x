#[doc = "Register `txdata` reader"]
pub struct R(crate::R<TXDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `txdata` writer"]
pub struct W(crate::W<TXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA_SPEC>;
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
impl From<crate::W<TXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data` reader - Transmit data"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `data` writer - Transmit data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `full` reader - FIFO full flag"]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `full` writer - FIFO full flag"]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXDATA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - FIFO full flag"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 31 - FIFO full flag"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W<31> {
        FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](index.html) module"]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdata::R](R) reader structure"]
impl crate::Readable for TXDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdata::W](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets txdata to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
