#[doc = "Reader of register txmark"]
pub type R = crate::R<u32, super::TXMARK>;
#[doc = "Writer for register txmark"]
pub type W = crate::W<u32, super::TXMARK>;
#[doc = "Register txmark `reset()`'s with value 0"]
impl crate::ResetValue for super::TXMARK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `txmark`"]
pub type TXMARK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `txmark`"]
pub struct TXMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    pub fn txmark(&self) -> TXMARK_R {
        TXMARK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit watermark"]
    #[inline(always)]
    pub fn txmark(&mut self) -> TXMARK_W {
        TXMARK_W { w: self }
    }
}
