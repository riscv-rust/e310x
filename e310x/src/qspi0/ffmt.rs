#[doc = "Register `ffmt` reader"]
pub type R = crate::R<FfmtSpec>;
#[doc = "Register `ffmt` writer"]
pub type W = crate::W<FfmtSpec>;
#[doc = "Field `cmd_en` reader - Enable sending of command"]
pub type CmdEnR = crate::BitReader;
#[doc = "Field `cmd_en` writer - Enable sending of command"]
pub type CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addr_len` reader - Number of address bytes (0 to 4)"]
pub type AddrLenR = crate::FieldReader;
#[doc = "Field `addr_len` writer - Number of address bytes (0 to 4)"]
pub type AddrLenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pad_cnt` reader - Number of dummy cycles"]
pub type PadCntR = crate::FieldReader;
#[doc = "Field `pad_cnt` writer - Number of dummy cycles"]
pub type PadCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Protocol for transmitting command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmdProto {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    Single = 0,
    #[doc = "1: DQ0, DQ1"]
    Dual = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    Quad = 2,
}
impl From<CmdProto> for u8 {
    #[inline(always)]
    fn from(variant: CmdProto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmdProto {
    type Ux = u8;
}
impl crate::IsEnum for CmdProto {}
#[doc = "Field `cmd_proto` reader - Protocol for transmitting command"]
pub type CmdProtoR = crate::FieldReader<CmdProto>;
impl CmdProtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CmdProto> {
        match self.bits {
            0 => Some(CmdProto::Single),
            1 => Some(CmdProto::Dual),
            2 => Some(CmdProto::Quad),
            _ => None,
        }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CmdProto::Single
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == CmdProto::Dual
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == CmdProto::Quad
    }
}
#[doc = "Field `cmd_proto` writer - Protocol for transmitting command"]
pub type CmdProtoW<'a, REG> = crate::FieldWriter<'a, REG, 2, CmdProto>;
impl<'a, REG> CmdProtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CmdProto::Single)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(CmdProto::Dual)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(CmdProto::Quad)
    }
}
#[doc = "Protocol for transmitting address and padding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AddrProto {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    Single = 0,
    #[doc = "1: DQ0, DQ1"]
    Dual = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    Quad = 2,
}
impl From<AddrProto> for u8 {
    #[inline(always)]
    fn from(variant: AddrProto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AddrProto {
    type Ux = u8;
}
impl crate::IsEnum for AddrProto {}
#[doc = "Field `addr_proto` reader - Protocol for transmitting address and padding"]
pub type AddrProtoR = crate::FieldReader<AddrProto>;
impl AddrProtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AddrProto> {
        match self.bits {
            0 => Some(AddrProto::Single),
            1 => Some(AddrProto::Dual),
            2 => Some(AddrProto::Quad),
            _ => None,
        }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AddrProto::Single
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == AddrProto::Dual
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == AddrProto::Quad
    }
}
#[doc = "Field `addr_proto` writer - Protocol for transmitting address and padding"]
pub type AddrProtoW<'a, REG> = crate::FieldWriter<'a, REG, 2, AddrProto>;
impl<'a, REG> AddrProtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(AddrProto::Single)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(AddrProto::Dual)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(AddrProto::Quad)
    }
}
#[doc = "Protocol for receiving data bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataProto {
    #[doc = "0: DQ0 (MOSI), DQ1 (MISO)"]
    Single = 0,
    #[doc = "1: DQ0, DQ1"]
    Dual = 1,
    #[doc = "2: DQ0, DQ1, DQ2, DQ3"]
    Quad = 2,
}
impl From<DataProto> for u8 {
    #[inline(always)]
    fn from(variant: DataProto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataProto {
    type Ux = u8;
}
impl crate::IsEnum for DataProto {}
#[doc = "Field `data_proto` reader - Protocol for receiving data bytes"]
pub type DataProtoR = crate::FieldReader<DataProto>;
impl DataProtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DataProto> {
        match self.bits {
            0 => Some(DataProto::Single),
            1 => Some(DataProto::Dual),
            2 => Some(DataProto::Quad),
            _ => None,
        }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DataProto::Single
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DataProto::Dual
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DataProto::Quad
    }
}
#[doc = "Field `data_proto` writer - Protocol for receiving data bytes"]
pub type DataProtoW<'a, REG> = crate::FieldWriter<'a, REG, 2, DataProto>;
impl<'a, REG> DataProtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(DataProto::Single)
    }
    #[doc = "DQ0, DQ1"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(DataProto::Dual)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(DataProto::Quad)
    }
}
#[doc = "Field `cmd_code` reader - Value of command byte"]
pub type CmdCodeR = crate::FieldReader;
#[doc = "Field `cmd_code` writer - Value of command byte"]
pub type CmdCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pad_code` reader - First 8 bits to transmit during dummy cycles"]
pub type PadCodeR = crate::FieldReader;
#[doc = "Field `pad_code` writer - First 8 bits to transmit during dummy cycles"]
pub type PadCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline(always)]
    pub fn cmd_en(&self) -> CmdEnR {
        CmdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline(always)]
    pub fn addr_len(&self) -> AddrLenR {
        AddrLenR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Number of dummy cycles"]
    #[inline(always)]
    pub fn pad_cnt(&self) -> PadCntR {
        PadCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline(always)]
    pub fn cmd_proto(&self) -> CmdProtoR {
        CmdProtoR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline(always)]
    pub fn addr_proto(&self) -> AddrProtoR {
        AddrProtoR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline(always)]
    pub fn data_proto(&self) -> DataProtoR {
        DataProtoR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline(always)]
    pub fn cmd_code(&self) -> CmdCodeR {
        CmdCodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline(always)]
    pub fn pad_code(&self) -> PadCodeR {
        PadCodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_en(&mut self) -> CmdEnW<FfmtSpec> {
        CmdEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline(always)]
    #[must_use]
    pub fn addr_len(&mut self) -> AddrLenW<FfmtSpec> {
        AddrLenW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Number of dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cnt(&mut self) -> PadCntW<FfmtSpec> {
        PadCntW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_proto(&mut self) -> CmdProtoW<FfmtSpec> {
        CmdProtoW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline(always)]
    #[must_use]
    pub fn addr_proto(&mut self) -> AddrProtoW<FfmtSpec> {
        AddrProtoW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline(always)]
    #[must_use]
    pub fn data_proto(&mut self) -> DataProtoW<FfmtSpec> {
        DataProtoW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_code(&mut self) -> CmdCodeW<FfmtSpec> {
        CmdCodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn pad_code(&mut self) -> PadCodeW<FfmtSpec> {
        PadCodeW::new(self, 24)
    }
}
#[doc = "SPI Flash Instruction Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfmtSpec;
impl crate::RegisterSpec for FfmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffmt::R`](R) reader structure"]
impl crate::Readable for FfmtSpec {}
#[doc = "`write(|w| ..)` method takes [`ffmt::W`](W) writer structure"]
impl crate::Writable for FfmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ffmt to value 0"]
impl crate::Resettable for FfmtSpec {
    const RESET_VALUE: u32 = 0;
}
