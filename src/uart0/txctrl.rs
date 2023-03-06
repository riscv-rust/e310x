#[doc = "Register `txctrl` reader"]
pub struct R(crate::R<TXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `txctrl` writer"]
pub struct W(crate::W<TXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL_SPEC>;
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
impl From<crate::W<TXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTRL_SPEC, bool, O>;
#[doc = "Field `nstop` reader - "]
pub type NSTOP_R = crate::BitReader<bool>;
#[doc = "Field `nstop` writer - "]
pub type NSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTRL_SPEC, bool, O>;
#[doc = "Field `counter` reader - "]
pub type COUNTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `counter` writer - "]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&self) -> NSTOP_R {
        NSTOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&mut self) -> NSTOP_W<1> {
        NSTOP_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W<16> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl](index.html) module"]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl::R](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl::W](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets txctrl to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
