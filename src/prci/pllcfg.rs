#[doc = "Register `pllcfg` reader"]
pub type R = crate::R<PLLCFG_SPEC>;
#[doc = "Register `pllcfg` writer"]
pub type W = crate::W<PLLCFG_SPEC>;
#[doc = "Field `pllr` reader - "]
pub type PLLR_R = crate::FieldReader<PLLR_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLR_A {
    #[doc = "0: `0`"]
    R1 = 0,
    #[doc = "1: `1`"]
    R2 = 1,
    #[doc = "2: `10`"]
    R3 = 2,
    #[doc = "3: `11`"]
    R4 = 3,
}
impl From<PLLR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLR_A {
    type Ux = u8;
}
impl PLLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLR_A> {
        match self.bits {
            0 => Some(PLLR_A::R1),
            1 => Some(PLLR_A::R2),
            2 => Some(PLLR_A::R3),
            3 => Some(PLLR_A::R4),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_r1(&self) -> bool {
        *self == PLLR_A::R1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_r2(&self) -> bool {
        *self == PLLR_A::R2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_r3(&self) -> bool {
        *self == PLLR_A::R3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_r4(&self) -> bool {
        *self == PLLR_A::R4
    }
}
#[doc = "Field `pllr` writer - "]
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLLR_A>;
impl<'a, REG> PLLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR_A::R1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR_A::R2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR_A::R3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR_A::R4)
    }
}
#[doc = "Field `pllf` reader - "]
pub type PLLF_R = crate::FieldReader;
#[doc = "Field `pllf` writer - "]
pub type PLLF_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pllq` reader - "]
pub type PLLQ_R = crate::FieldReader<PLLQ_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ_A {
    #[doc = "1: `1`"]
    Q2 = 1,
    #[doc = "2: `10`"]
    Q4 = 2,
    #[doc = "3: `11`"]
    Q8 = 3,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLQ_A {
    type Ux = u8;
}
impl PLLQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLQ_A> {
        match self.bits {
            1 => Some(PLLQ_A::Q2),
            2 => Some(PLLQ_A::Q4),
            3 => Some(PLLQ_A::Q8),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_q2(&self) -> bool {
        *self == PLLQ_A::Q2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_q4(&self) -> bool {
        *self == PLLQ_A::Q4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_q8(&self) -> bool {
        *self == PLLQ_A::Q8
    }
}
#[doc = "Field `pllq` writer - "]
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLQ_A>;
impl<'a, REG> PLLQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn q2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ_A::Q2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn q4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ_A::Q4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn q8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ_A::Q8)
    }
}
#[doc = "Field `sel` reader - "]
pub type SEL_R = crate::BitReader;
#[doc = "Field `sel` writer - "]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refsel` reader - "]
pub type REFSEL_R = crate::BitReader;
#[doc = "Field `refsel` writer - "]
pub type REFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypass` reader - "]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `bypass` writer - "]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lock` reader - "]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `lock` writer - "]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn pllf(&self) -> PLLF_R {
        PLLF_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLCFG_SPEC> {
        PLLR_W::new(self, 0)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    #[must_use]
    pub fn pllf(&mut self) -> PLLF_W<PLLCFG_SPEC> {
        PLLF_W::new(self, 4)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLCFG_SPEC> {
        PLLQ_W::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<PLLCFG_SPEC> {
        SEL_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<PLLCFG_SPEC> {
        REFSEL_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<PLLCFG_SPEC> {
        BYPASS_W::new(self, 18)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<PLLCFG_SPEC> {
        LOCK_W::new(self, 31)
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
#[doc = "PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFG_SPEC;
impl crate::RegisterSpec for PLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfg::R`](R) reader structure"]
impl crate::Readable for PLLCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcfg::W`](W) writer structure"]
impl crate::Writable for PLLCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pllcfg to value 0x0003_06f9"]
impl crate::Resettable for PLLCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_06f9;
}
