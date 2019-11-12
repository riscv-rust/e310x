#[doc = "Reader of register wdogcmp"]
pub type R = crate::R<u32, super::WDOGCMP>;
#[doc = "Writer for register wdogcmp"]
pub type W = crate::W<u32, super::WDOGCMP>;
#[doc = "Register wdogcmp `reset()`'s with value 0"]
impl crate::ResetValue for super::WDOGCMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `value`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `value`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
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
}
