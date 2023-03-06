#[doc = "Register `ip` reader"]
pub struct R(crate::R<IP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ip` writer"]
pub struct W(crate::W<IP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IP_SPEC>;
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
impl From<crate::W<IP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txwm` reader - "]
pub type TXWM_R = crate::BitReader<bool>;
#[doc = "Field `txwm` writer - "]
pub type TXWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IP_SPEC, bool, O>;
#[doc = "Field `rxwm` reader - "]
pub type RXWM_R = crate::BitReader<bool>;
#[doc = "Field `rxwm` writer - "]
pub type RXWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txwm(&self) -> TXWM_R {
        TXWM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxwm(&self) -> RXWM_R {
        RXWM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txwm(&mut self) -> TXWM_W<0> {
        TXWM_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxwm(&mut self) -> RXWM_W<1> {
        RXWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip](index.html) module"]
pub struct IP_SPEC;
impl crate::RegisterSpec for IP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ip::R](R) reader structure"]
impl crate::Readable for IP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ip::W](W) writer structure"]
impl crate::Writable for IP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ip to value 0"]
impl crate::Resettable for IP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
