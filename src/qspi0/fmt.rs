#[doc = "Register `fmt` reader"]
pub type R = crate::R<FmtSpec>;
#[doc = "Register `fmt` writer"]
pub type W = crate::W<FmtSpec>;
#[doc = "SPI protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Proto {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    Single = 0,
    #[doc = "1: DQ0, DQ1"]
    Dual = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    Quad = 2,
}
impl From<Proto> for u8 {
    #[inline(always)]
    fn from(variant: Proto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Proto {
    type Ux = u8;
}
impl crate::IsEnum for Proto {}
#[doc = "Field `proto` reader - SPI protocol"]
pub type ProtoR = crate::FieldReader<Proto>;
impl ProtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Proto> {
        match self.bits {
            0 => Some(Proto::Single),
            1 => Some(Proto::Dual),
            2 => Some(Proto::Quad),
            _ => None,
        }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Proto::Single
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Proto::Dual
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Proto::Quad
    }
}
#[doc = "Field `proto` writer - SPI protocol"]
pub type ProtoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Proto>;
impl<'a, REG> ProtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Proto::Single)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(Proto::Dual)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Proto::Quad)
    }
}
#[doc = "SPI endianness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Transmit MSB first."]
    Big = 0,
    #[doc = "1: Transmit LSB first."]
    Little = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `endian` reader - SPI endianness"]
pub type EndianR = crate::BitReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::Big,
            true => Endian::Little,
        }
    }
    #[doc = "Transmit MSB first."]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == Endian::Big
    }
    #[doc = "Transmit LSB first."]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == Endian::Little
    }
}
#[doc = "Field `endian` writer - SPI endianness"]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit MSB first."]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Big)
    }
    #[doc = "Transmit LSB first."]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Little)
    }
}
#[doc = "SPI I/O direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    Rx = 0,
    #[doc = "1: The receive FIFO is not populated."]
    Tx = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dir` reader - SPI I/O direction"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Rx,
            true => Dir::Tx,
        }
    }
    #[doc = "For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == Dir::Rx
    }
    #[doc = "The receive FIFO is not populated."]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == Dir::Tx
    }
}
#[doc = "Field `dir` writer - SPI I/O direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Rx)
    }
    #[doc = "The receive FIFO is not populated."]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Tx)
    }
}
#[doc = "Field `len` reader - Number of bits per frame"]
pub type LenR = crate::FieldReader;
#[doc = "Field `len` writer - Number of bits per frame"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&self) -> ProtoR {
        ProtoR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    #[must_use]
    pub fn proto(&mut self) -> ProtoW<FmtSpec> {
        ProtoW::new(self, 0)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<FmtSpec> {
        EndianW::new(self, 2)
    }
    #[doc = "Bit 3 - SPI I/O direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<FmtSpec> {
        DirW::new(self, 3)
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<FmtSpec> {
        LenW::new(self, 16)
    }
}
#[doc = "Frame Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmtSpec;
impl crate::RegisterSpec for FmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmt::R`](R) reader structure"]
impl crate::Readable for FmtSpec {}
#[doc = "`write(|w| ..)` method takes [`fmt::W`](W) writer structure"]
impl crate::Writable for FmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fmt to value 0"]
impl crate::Resettable for FmtSpec {
    const RESET_VALUE: u32 = 0;
}
