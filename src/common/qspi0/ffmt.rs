#[doc = "Reader of register ffmt"]
pub type R = crate::R<u32, super::FFMT>;
#[doc = "Writer for register ffmt"]
pub type W = crate::W<u32, super::FFMT>;
#[doc = "Register ffmt `reset()`'s with value 0"]
impl crate::ResetValue for super::FFMT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pad_code`"]
pub type PAD_CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pad_code`"]
pub struct PAD_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `cmd_code`"]
pub type CMD_CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cmd_code`"]
pub struct CMD_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Protocol for receiving data bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_PROTO_A {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE = 0,
    #[doc = "1: DQ0, DQ1"]
    DUAL = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    QUAD = 2,
}
impl From<DATA_PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_PROTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `data_proto`"]
pub type DATA_PROTO_R = crate::R<u8, DATA_PROTO_A>;
impl DATA_PROTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATA_PROTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATA_PROTO_A::SINGLE),
            1 => Val(DATA_PROTO_A::DUAL),
            2 => Val(DATA_PROTO_A::QUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DATA_PROTO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATA_PROTO_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DATA_PROTO_A::QUAD
    }
}
#[doc = "Write proxy for field `data_proto`"]
pub struct DATA_PROTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PROTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_PROTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DATA_PROTO_A::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DATA_PROTO_A::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(DATA_PROTO_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Protocol for transmitting address and padding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDR_PROTO_A {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE = 0,
    #[doc = "1: DQ0, DQ1"]
    DUAL = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    QUAD = 2,
}
impl From<ADDR_PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDR_PROTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `addr_proto`"]
pub type ADDR_PROTO_R = crate::R<u8, ADDR_PROTO_A>;
impl ADDR_PROTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADDR_PROTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADDR_PROTO_A::SINGLE),
            1 => Val(ADDR_PROTO_A::DUAL),
            2 => Val(ADDR_PROTO_A::QUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ADDR_PROTO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == ADDR_PROTO_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == ADDR_PROTO_A::QUAD
    }
}
#[doc = "Write proxy for field `addr_proto`"]
pub struct ADDR_PROTO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_PROTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_PROTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ADDR_PROTO_A::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(ADDR_PROTO_A::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(ADDR_PROTO_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Protocol for transmitting command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_PROTO_A {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE = 0,
    #[doc = "1: DQ0, DQ1"]
    DUAL = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    QUAD = 2,
}
impl From<CMD_PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_PROTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `cmd_proto`"]
pub type CMD_PROTO_R = crate::R<u8, CMD_PROTO_A>;
impl CMD_PROTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_PROTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMD_PROTO_A::SINGLE),
            1 => Val(CMD_PROTO_A::DUAL),
            2 => Val(CMD_PROTO_A::QUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CMD_PROTO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == CMD_PROTO_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == CMD_PROTO_A::QUAD
    }
}
#[doc = "Write proxy for field `cmd_proto`"]
pub struct CMD_PROTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_PROTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_PROTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CMD_PROTO_A::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(CMD_PROTO_A::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(CMD_PROTO_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `pad_cnt`"]
pub type PAD_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pad_cnt`"]
pub struct PAD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `addr_len`"]
pub type ADDR_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `addr_len`"]
pub struct ADDR_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `cmd_en`"]
pub type CMD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmd_en`"]
pub struct CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline(always)]
    pub fn pad_code(&self) -> PAD_CODE_R {
        PAD_CODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline(always)]
    pub fn cmd_code(&self) -> CMD_CODE_R {
        CMD_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline(always)]
    pub fn data_proto(&self) -> DATA_PROTO_R {
        DATA_PROTO_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline(always)]
    pub fn addr_proto(&self) -> ADDR_PROTO_R {
        ADDR_PROTO_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline(always)]
    pub fn cmd_proto(&self) -> CMD_PROTO_R {
        CMD_PROTO_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Number of dummy cycles"]
    #[inline(always)]
    pub fn pad_cnt(&self) -> PAD_CNT_R {
        PAD_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline(always)]
    pub fn addr_len(&self) -> ADDR_LEN_R {
        ADDR_LEN_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline(always)]
    pub fn cmd_en(&self) -> CMD_EN_R {
        CMD_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline(always)]
    pub fn pad_code(&mut self) -> PAD_CODE_W {
        PAD_CODE_W { w: self }
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline(always)]
    pub fn cmd_code(&mut self) -> CMD_CODE_W {
        CMD_CODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline(always)]
    pub fn data_proto(&mut self) -> DATA_PROTO_W {
        DATA_PROTO_W { w: self }
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline(always)]
    pub fn addr_proto(&mut self) -> ADDR_PROTO_W {
        ADDR_PROTO_W { w: self }
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline(always)]
    pub fn cmd_proto(&mut self) -> CMD_PROTO_W {
        CMD_PROTO_W { w: self }
    }
    #[doc = "Bits 4:7 - Number of dummy cycles"]
    #[inline(always)]
    pub fn pad_cnt(&mut self) -> PAD_CNT_W {
        PAD_CNT_W { w: self }
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline(always)]
    pub fn addr_len(&mut self) -> ADDR_LEN_W {
        ADDR_LEN_W { w: self }
    }
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline(always)]
    pub fn cmd_en(&mut self) -> CMD_EN_W {
        CMD_EN_W { w: self }
    }
}
