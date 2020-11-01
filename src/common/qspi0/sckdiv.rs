#[doc = "Reader of register sckdiv"]
pub type R = crate::R<u32, super::SCKDIV>;
#[doc = "Writer for register sckdiv"]
pub type W = crate::W<u32, super::SCKDIV>;
#[doc = "Register sckdiv `reset()`'s with value 0"]
impl crate::ResetValue for super::SCKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `div`"]
pub type DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `div`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Divisor for serial clock"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Divisor for serial clock"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
