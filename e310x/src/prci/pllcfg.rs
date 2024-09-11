#[doc = "Register `pllcfg` reader"]
pub type R = crate::R<PllcfgSpec>;
#[doc = "Register `pllcfg` writer"]
pub type W = crate::W<PllcfgSpec>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllr {
    #[doc = "0: `0`"]
    R1 = 0,
    #[doc = "1: `1`"]
    R2 = 1,
    #[doc = "2: `10`"]
    R3 = 2,
    #[doc = "3: `11`"]
    R4 = 3,
}
impl From<Pllr> for u8 {
    #[inline(always)]
    fn from(variant: Pllr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllr {
    type Ux = u8;
}
impl crate::IsEnum for Pllr {}
#[doc = "Field `pllr` reader - "]
pub type PllrR = crate::FieldReader<Pllr>;
impl PllrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllr> {
        match self.bits {
            0 => Some(Pllr::R1),
            1 => Some(Pllr::R2),
            2 => Some(Pllr::R3),
            3 => Some(Pllr::R4),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_r1(&self) -> bool {
        *self == Pllr::R1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_r2(&self) -> bool {
        *self == Pllr::R2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_r3(&self) -> bool {
        *self == Pllr::R3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_r4(&self) -> bool {
        *self == Pllr::R4
    }
}
#[doc = "Field `pllr` writer - "]
pub type PllrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pllr>;
impl<'a, REG> PllrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::R1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::R2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::R3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::R4)
    }
}
#[doc = "Field `pllf` reader - "]
pub type PllfR = crate::FieldReader;
#[doc = "Field `pllf` writer - "]
pub type PllfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllq {
    #[doc = "1: `1`"]
    Q2 = 1,
    #[doc = "2: `10`"]
    Q4 = 2,
    #[doc = "3: `11`"]
    Q8 = 3,
}
impl From<Pllq> for u8 {
    #[inline(always)]
    fn from(variant: Pllq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllq {
    type Ux = u8;
}
impl crate::IsEnum for Pllq {}
#[doc = "Field `pllq` reader - "]
pub type PllqR = crate::FieldReader<Pllq>;
impl PllqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllq> {
        match self.bits {
            1 => Some(Pllq::Q2),
            2 => Some(Pllq::Q4),
            3 => Some(Pllq::Q8),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_q2(&self) -> bool {
        *self == Pllq::Q2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_q4(&self) -> bool {
        *self == Pllq::Q4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_q8(&self) -> bool {
        *self == Pllq::Q8
    }
}
#[doc = "Field `pllq` writer - "]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllq>;
impl<'a, REG> PllqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn q2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Q2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn q4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Q4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn q8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Q8)
    }
}
#[doc = "Field `sel` reader - "]
pub type SelR = crate::BitReader;
#[doc = "Field `sel` writer - "]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refsel` reader - "]
pub type RefselR = crate::BitReader;
#[doc = "Field `refsel` writer - "]
pub type RefselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass` reader - "]
pub type BypassR = crate::BitReader;
#[doc = "Field `bypass` writer - "]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lock` reader - "]
pub type LockR = crate::BitReader;
#[doc = "Field `lock` writer - "]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pllr(&self) -> PllrR {
        PllrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn pllf(&self) -> PllfR {
        PllfR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PllrW<PllcfgSpec> {
        PllrW::new(self, 0)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    #[must_use]
    pub fn pllf(&mut self) -> PllfW<PllcfgSpec> {
        PllfW::new(self, 4)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PllqW<PllcfgSpec> {
        PllqW::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<PllcfgSpec> {
        SelW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> RefselW<PllcfgSpec> {
        RefselW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<PllcfgSpec> {
        BypassW::new(self, 18)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<PllcfgSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgSpec;
impl crate::RegisterSpec for PllcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfg::R`](R) reader structure"]
impl crate::Readable for PllcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfg::W`](W) writer structure"]
impl crate::Writable for PllcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pllcfg to value 0x0003_06f9"]
impl crate::Resettable for PllcfgSpec {
    const RESET_VALUE: u32 = 0x0003_06f9;
}
