#[doc = "Register `fctrl` reader"]
pub struct R(crate::R<FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fctrl` writer"]
pub struct W(crate::W<FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTRL_SPEC>;
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
impl From<crate::W<FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en` reader - SPI Flash Mode Select"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `en` writer - SPI Flash Mode Select"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI Flash Mode Select"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Flash Mode Select"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Flash Interface Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl](index.html) module"]
pub struct FCTRL_SPEC;
impl crate::RegisterSpec for FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctrl::R](R) reader structure"]
impl crate::Readable for FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctrl::W](W) writer structure"]
impl crate::Writable for FCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fctrl to value 0"]
impl crate::Resettable for FCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
