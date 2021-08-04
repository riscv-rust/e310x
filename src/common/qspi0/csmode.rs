#[doc = "Register `csmode` reader"]
pub struct R(crate::R<CSMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csmode` writer"]
pub struct W(crate::W<CSMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSMODE_SPEC>;
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
impl From<crate::W<CSMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Chip select mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: \n                    Assert/de-assert CS at the beginning/end of each frame.\n                  "]
    AUTO = 0,
    #[doc = "2: \n                    Keep CS continuously asserted after the initial frame.\n                  "]
    HOLD = 2,
    #[doc = "3: Disable hardware control of the CS pin."]
    OFF = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `mode` reader - Chip select mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::AUTO),
            2 => Some(MODE_A::HOLD),
            3 => Some(MODE_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        **self == MODE_A::AUTO
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        **self == MODE_A::HOLD
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == MODE_A::OFF
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode` writer - Chip select mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Assert/de-assert CS at the beginning/end of each frame."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(MODE_A::AUTO)
    }
    #[doc = "Keep CS continuously asserted after the initial frame."]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(MODE_A::HOLD)
    }
    #[doc = "Disable hardware control of the CS pin."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Chip select mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chip select mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csmode](index.html) module"]
pub struct CSMODE_SPEC;
impl crate::RegisterSpec for CSMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csmode::R](R) reader structure"]
impl crate::Readable for CSMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csmode::W](W) writer structure"]
impl crate::Writable for CSMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csmode to value 0"]
impl crate::Resettable for CSMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
