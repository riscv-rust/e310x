#[doc = "Register `lfrosccfg` reader"]
pub struct R(crate::R<LFROSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFROSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFROSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFROSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lfrosccfg` writer"]
pub struct W(crate::W<LFROSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFROSCCFG_SPEC>;
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
impl From<crate::W<LFROSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFROSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `div` reader - "]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `div` writer - "]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFROSCCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `trim` reader - "]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `trim` writer - "]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFROSCCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFROSCCFG_SPEC, bool, O>;
#[doc = "Field `ready` reader - "]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `ready` writer - "]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFROSCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
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
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<16> {
        TRIM_W::new(self)
    }
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
#[doc = "AON Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfrosccfg](index.html) module"]
pub struct LFROSCCFG_SPEC;
impl crate::RegisterSpec for LFROSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfrosccfg::R](R) reader structure"]
impl crate::Readable for LFROSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfrosccfg::W](W) writer structure"]
impl crate::Writable for LFROSCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lfrosccfg to value 0"]
impl crate::Resettable for LFROSCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
