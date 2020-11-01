#[doc = "Reader of register rxmark"]
pub type R = crate::R<u32, super::RXMARK>;
#[doc = "Writer for register rxmark"]
pub type W = crate::W<u32, super::RXMARK>;
#[doc = "Register rxmark `reset()`'s with value 0"]
impl crate::ResetValue for super::RXMARK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rxmark`"]
pub type RXMARK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rxmark`"]
pub struct RXMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    pub fn rxmark(&self) -> RXMARK_R {
        RXMARK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    pub fn rxmark(&mut self) -> RXMARK_W {
        RXMARK_W { w: self }
    }
}
