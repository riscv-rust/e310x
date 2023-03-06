#[doc = "Register `ffmt` reader"]
pub struct R(crate::R<FFMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ffmt` writer"]
pub struct W(crate::W<FFMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFMT_SPEC>;
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
impl From<crate::W<FFMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmd_en` reader - Enable sending of command"]
pub type CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cmd_en` writer - Enable sending of command"]
pub type CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFMT_SPEC, bool, O>;
#[doc = "Field `addr_len` reader - Number of address bytes (0 to 4)"]
pub type ADDR_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `addr_len` writer - Number of address bytes (0 to 4)"]
pub type ADDR_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFMT_SPEC, u8, u8, 3, O>;
#[doc = "Field `pad_cnt` reader - Number of dummy cycles"]
pub type PAD_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_cnt` writer - Number of dummy cycles"]
pub type PAD_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFMT_SPEC, u8, u8, 4, O>;
#[doc = "Field `cmd_proto` reader - Protocol for transmitting command"]
pub type CMD_PROTO_R = crate::FieldReader<u8, CMD_PROTO_A>;
#[doc = "Protocol for transmitting command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CMD_PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_PROTO_A> {
        match self.bits {
            0 => Some(CMD_PROTO_A::SINGLE),
            1 => Some(CMD_PROTO_A::DUAL),
            2 => Some(CMD_PROTO_A::QUAD),
            _ => None,
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
#[doc = "Field `cmd_proto` writer - Protocol for transmitting command"]
pub type CMD_PROTO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FFMT_SPEC, u8, CMD_PROTO_A, 2, O>;
impl<'a, const O: u8> CMD_PROTO_W<'a, O> {
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
}
#[doc = "Field `addr_proto` reader - Protocol for transmitting address and padding"]
pub type ADDR_PROTO_R = crate::FieldReader<u8, ADDR_PROTO_A>;
#[doc = "Protocol for transmitting address and padding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ADDR_PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDR_PROTO_A> {
        match self.bits {
            0 => Some(ADDR_PROTO_A::SINGLE),
            1 => Some(ADDR_PROTO_A::DUAL),
            2 => Some(ADDR_PROTO_A::QUAD),
            _ => None,
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
#[doc = "Field `addr_proto` writer - Protocol for transmitting address and padding"]
pub type ADDR_PROTO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FFMT_SPEC, u8, ADDR_PROTO_A, 2, O>;
impl<'a, const O: u8> ADDR_PROTO_W<'a, O> {
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
}
#[doc = "Field `data_proto` reader - Protocol for receiving data bytes"]
pub type DATA_PROTO_R = crate::FieldReader<u8, DATA_PROTO_A>;
#[doc = "Protocol for receiving data bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DATA_PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATA_PROTO_A> {
        match self.bits {
            0 => Some(DATA_PROTO_A::SINGLE),
            1 => Some(DATA_PROTO_A::DUAL),
            2 => Some(DATA_PROTO_A::QUAD),
            _ => None,
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
#[doc = "Field `data_proto` writer - Protocol for receiving data bytes"]
pub type DATA_PROTO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FFMT_SPEC, u8, DATA_PROTO_A, 2, O>;
impl<'a, const O: u8> DATA_PROTO_W<'a, O> {
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
}
#[doc = "Field `cmd_code` reader - Value of command byte"]
pub type CMD_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cmd_code` writer - Value of command byte"]
pub type CMD_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFMT_SPEC, u8, u8, 8, O>;
#[doc = "Field `pad_code` reader - First 8 bits to transmit during dummy cycles"]
pub type PAD_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_code` writer - First 8 bits to transmit during dummy cycles"]
pub type PAD_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFMT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline(always)]
    pub fn cmd_en(&self) -> CMD_EN_R {
        CMD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline(always)]
    pub fn addr_len(&self) -> ADDR_LEN_R {
        ADDR_LEN_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Number of dummy cycles"]
    #[inline(always)]
    pub fn pad_cnt(&self) -> PAD_CNT_R {
        PAD_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline(always)]
    pub fn cmd_proto(&self) -> CMD_PROTO_R {
        CMD_PROTO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline(always)]
    pub fn addr_proto(&self) -> ADDR_PROTO_R {
        ADDR_PROTO_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline(always)]
    pub fn data_proto(&self) -> DATA_PROTO_R {
        DATA_PROTO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline(always)]
    pub fn cmd_code(&self) -> CMD_CODE_R {
        CMD_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline(always)]
    pub fn pad_code(&self) -> PAD_CODE_R {
        PAD_CODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline(always)]
    pub fn cmd_en(&mut self) -> CMD_EN_W<0> {
        CMD_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline(always)]
    pub fn addr_len(&mut self) -> ADDR_LEN_W<1> {
        ADDR_LEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Number of dummy cycles"]
    #[inline(always)]
    pub fn pad_cnt(&mut self) -> PAD_CNT_W<4> {
        PAD_CNT_W::new(self)
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline(always)]
    pub fn cmd_proto(&mut self) -> CMD_PROTO_W<8> {
        CMD_PROTO_W::new(self)
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline(always)]
    pub fn addr_proto(&mut self) -> ADDR_PROTO_W<10> {
        ADDR_PROTO_W::new(self)
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline(always)]
    pub fn data_proto(&mut self) -> DATA_PROTO_W<12> {
        DATA_PROTO_W::new(self)
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline(always)]
    pub fn cmd_code(&mut self) -> CMD_CODE_W<16> {
        CMD_CODE_W::new(self)
    }
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline(always)]
    pub fn pad_code(&mut self) -> PAD_CODE_W<24> {
        PAD_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Flash Instruction Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffmt](index.html) module"]
pub struct FFMT_SPEC;
impl crate::RegisterSpec for FFMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffmt::R](R) reader structure"]
impl crate::Readable for FFMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffmt::W](W) writer structure"]
impl crate::Writable for FFMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ffmt to value 0"]
impl crate::Resettable for FFMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
