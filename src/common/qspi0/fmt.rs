#[doc = "Reader of register fmt"]
pub type R = crate::R<u32, super::FMT>;
#[doc = "Writer for register fmt"]
pub type W = crate::W<u32, super::FMT>;
#[doc = "Register fmt `reset()`'s with value 0"]
impl crate::ResetValue for super::FMT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `len`"]
pub type LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `len`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "SPI I/O direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: \n                    For dual and quad protocols, the DQ pins are tri-stated. For\n                    the single protocol, the DQ0 pin is driven with the transmit\n                    data as normal.\n                  "]
    RX = 0,
    #[doc = "1: The receive FIFO is not populated."]
    TX = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `dir`"]
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::RX,
            true => DIR_A::TX,
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == DIR_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == DIR_A::TX
    }
}
#[doc = "Write proxy for field `dir`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(DIR_A::RX)
    }
    #[doc = "The receive FIFO is not populated."]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(DIR_A::TX)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "SPI endianness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIAN_A {
    #[doc = "0: Transmit MSB first."]
    BIG = 0,
    #[doc = "1: Transmit LSB first."]
    LITTLE = 1,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `endian`"]
pub type ENDIAN_R = crate::R<bool, ENDIAN_A>;
impl ENDIAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::BIG,
            true => ENDIAN_A::LITTLE,
        }
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIAN_A::BIG
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIAN_A::LITTLE
    }
}
#[doc = "Write proxy for field `endian`"]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit MSB first."]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIAN_A::BIG)
    }
    #[doc = "Transmit LSB first."]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIAN_A::LITTLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "SPI protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROTO_A {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE = 0,
    #[doc = "1: DQ0, DQ1"]
    DUAL = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    QUAD = 2,
}
impl From<PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: PROTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `proto`"]
pub type PROTO_R = crate::R<u8, PROTO_A>;
impl PROTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PROTO_A::SINGLE),
            1 => Val(PROTO_A::DUAL),
            2 => Val(PROTO_A::QUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PROTO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == PROTO_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == PROTO_A::QUAD
    }
}
#[doc = "Write proxy for field `proto`"]
pub struct PROTO_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(PROTO_A::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(PROTO_A::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(PROTO_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&self) -> PROTO_R {
        PROTO_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&mut self) -> PROTO_W {
        PROTO_W { w: self }
    }
}
