#[doc = "Reader of register csmode"]
pub type R = crate::R<u32, super::CSMODE>;
#[doc = "Writer for register csmode"]
pub type W = crate::W<u32, super::CSMODE>;
#[doc = "Register csmode `reset()`'s with value 0"]
impl crate::ResetValue for super::CSMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `mode`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::AUTO),
            2 => Val(MODE_A::HOLD),
            3 => Val(MODE_A::OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == MODE_A::AUTO
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == MODE_A::HOLD
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
}
#[doc = "Write proxy for field `mode`"]
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
}
