#[doc = "Register `fmt` reader"]
pub struct R(crate::R<FMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fmt` writer"]
pub struct W(crate::W<FMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMT_SPEC>;
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
impl From<crate::W<FMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `proto` reader - SPI protocol"]
pub type PROTO_R = crate::FieldReader<u8, PROTO_A>;
#[doc = "SPI protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROTO_A> {
        match self.bits {
            0 => Some(PROTO_A::SINGLE),
            1 => Some(PROTO_A::DUAL),
            2 => Some(PROTO_A::QUAD),
            _ => None,
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
#[doc = "Field `proto` writer - SPI protocol"]
pub type PROTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMT_SPEC, u8, PROTO_A, 2, O>;
impl<'a, const O: u8> PROTO_W<'a, O> {
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
}
#[doc = "Field `endian` reader - SPI endianness"]
pub type ENDIAN_R = crate::BitReader<ENDIAN_A>;
#[doc = "SPI endianness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ENDIAN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `endian` writer - SPI endianness"]
pub type ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMT_SPEC, ENDIAN_A, O>;
impl<'a, const O: u8> ENDIAN_W<'a, O> {
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
}
#[doc = "Field `dir` reader - SPI I/O direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "SPI I/O direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
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
impl DIR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `dir` writer - SPI I/O direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMT_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
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
}
#[doc = "Field `len` reader - Number of bits per frame"]
pub type LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `len` writer - Number of bits per frame"]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&self) -> PROTO_R {
        PROTO_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&mut self) -> PROTO_W<0> {
        PROTO_W::new(self)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W<2> {
        ENDIAN_W::new(self)
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<3> {
        DIR_W::new(self)
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<16> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmt](index.html) module"]
pub struct FMT_SPEC;
impl crate::RegisterSpec for FMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmt::R](R) reader structure"]
impl crate::Readable for FMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmt::W](W) writer structure"]
impl crate::Writable for FMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fmt to value 0"]
impl crate::Resettable for FMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
