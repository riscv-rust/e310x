#[doc = "Register `fmt` reader"]
pub type R = crate::R<FMT_SPEC>;
#[doc = "Register `fmt` writer"]
pub type W = crate::W<FMT_SPEC>;
#[doc = "Field `proto` reader - SPI protocol"]
pub type PROTO_R = crate::FieldReader<PROTO_A>;
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
impl crate::FieldSpec for PROTO_A {
    type Ux = u8;
}
impl PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PROTO_A> {
        match self.bits {
            0 => Some(PROTO_A::SINGLE),
            1 => Some(PROTO_A::DUAL),
            2 => Some(PROTO_A::QUAD),
            _ => None,
        }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PROTO_A::SINGLE
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == PROTO_A::DUAL
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == PROTO_A::QUAD
    }
}
#[doc = "Field `proto` writer - SPI protocol"]
pub type PROTO_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PROTO_A>;
impl<'a, REG> PROTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(PROTO_A::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(PROTO_A::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::BIG,
            true => ENDIAN_A::LITTLE,
        }
    }
    #[doc = "Transmit MSB first."]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIAN_A::BIG
    }
    #[doc = "Transmit LSB first."]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIAN_A::LITTLE
    }
}
#[doc = "Field `endian` writer - SPI endianness"]
pub type ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG, ENDIAN_A>;
impl<'a, REG> ENDIAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit MSB first."]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIAN_A::BIG)
    }
    #[doc = "Transmit LSB first."]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::RX,
            true => DIR_A::TX,
        }
    }
    #[doc = "For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == DIR_A::RX
    }
    #[doc = "The receive FIFO is not populated."]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == DIR_A::TX
    }
}
#[doc = "Field `dir` writer - SPI I/O direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR_A>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::RX)
    }
    #[doc = "The receive FIFO is not populated."]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::TX)
    }
}
#[doc = "Field `len` reader - Number of bits per frame"]
pub type LEN_R = crate::FieldReader;
#[doc = "Field `len` writer - Number of bits per frame"]
pub type LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[must_use]
    pub fn proto(&mut self) -> PROTO_W<FMT_SPEC> {
        PROTO_W::new(self, 0)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> ENDIAN_W<FMT_SPEC> {
        ENDIAN_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<FMT_SPEC> {
        DIR_W::new(self, 3)
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<FMT_SPEC> {
        LEN_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frame Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMT_SPEC;
impl crate::RegisterSpec for FMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmt::R`](R) reader structure"]
impl crate::Readable for FMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmt::W`](W) writer structure"]
impl crate::Writable for FMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fmt to value 0"]
impl crate::Resettable for FMT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
