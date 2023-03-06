#[doc = "Register `pllcfg` reader"]
pub struct R(crate::R<PLLCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pllcfg` writer"]
pub struct W(crate::W<PLLCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFG_SPEC>;
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
impl From<crate::W<PLLCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pllr` reader - "]
pub type PLLR_R = crate::FieldReader<u8, PLLR_A>;
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
impl PLLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLR_A> {
        match self.bits {
            0 => Some(PLLR_A::R1),
            1 => Some(PLLR_A::R2),
            2 => Some(PLLR_A::R3),
            3 => Some(PLLR_A::R4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `R1`"]
    #[inline(always)]
    pub fn is_r1(&self) -> bool {
        *self == PLLR_A::R1
    }
    #[doc = "Checks if the value of the field is `R2`"]
    #[inline(always)]
    pub fn is_r2(&self) -> bool {
        *self == PLLR_A::R2
    }
    #[doc = "Checks if the value of the field is `R3`"]
    #[inline(always)]
    pub fn is_r3(&self) -> bool {
        *self == PLLR_A::R3
    }
    #[doc = "Checks if the value of the field is `R4`"]
    #[inline(always)]
    pub fn is_r4(&self) -> bool {
        *self == PLLR_A::R4
    }
}
#[doc = "Field `pllr` writer - "]
pub type PLLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFG_SPEC, u8, PLLR_A, 3, O>;
impl<'a, const O: u8> PLLR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r1(self) -> &'a mut W {
        self.variant(PLLR_A::R1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r2(self) -> &'a mut W {
        self.variant(PLLR_A::R2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r3(self) -> &'a mut W {
        self.variant(PLLR_A::R3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r4(self) -> &'a mut W {
        self.variant(PLLR_A::R4)
    }
}
#[doc = "Field `pllf` reader - "]
pub type PLLF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pllf` writer - "]
pub type PLLF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `pllq` reader - "]
pub type PLLQ_R = crate::FieldReader<u8, PLLQ_A>;
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
impl PLLQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLQ_A> {
        match self.bits {
            1 => Some(PLLQ_A::Q2),
            2 => Some(PLLQ_A::Q4),
            3 => Some(PLLQ_A::Q8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q2`"]
    #[inline(always)]
    pub fn is_q2(&self) -> bool {
        *self == PLLQ_A::Q2
    }
    #[doc = "Checks if the value of the field is `Q4`"]
    #[inline(always)]
    pub fn is_q4(&self) -> bool {
        *self == PLLQ_A::Q4
    }
    #[doc = "Checks if the value of the field is `Q8`"]
    #[inline(always)]
    pub fn is_q8(&self) -> bool {
        *self == PLLQ_A::Q8
    }
}
#[doc = "Field `pllq` writer - "]
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFG_SPEC, u8, PLLQ_A, 2, O>;
impl<'a, const O: u8> PLLQ_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn q2(self) -> &'a mut W {
        self.variant(PLLQ_A::Q2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn q4(self) -> &'a mut W {
        self.variant(PLLQ_A::Q4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn q8(self) -> &'a mut W {
        self.variant(PLLQ_A::Q8)
    }
}
#[doc = "Field `sel` reader - "]
pub type SEL_R = crate::BitReader<bool>;
#[doc = "Field `sel` writer - "]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFG_SPEC, bool, O>;
#[doc = "Field `refsel` reader - "]
pub type REFSEL_R = crate::BitReader<bool>;
#[doc = "Field `refsel` writer - "]
pub type REFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFG_SPEC, bool, O>;
#[doc = "Field `bypass` reader - "]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `bypass` writer - "]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFG_SPEC, bool, O>;
#[doc = "Field `lock` reader - "]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `lock` writer - "]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFG_SPEC, bool, O>;
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
    pub fn pllr(&mut self) -> PLLR_W<0> {
        PLLR_W::new(self)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn pllf(&mut self) -> PLLF_W<4> {
        PLLF_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<10> {
        PLLQ_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<16> {
        SEL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W<17> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<18> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfg](index.html) module"]
pub struct PLLCFG_SPEC;
impl crate::RegisterSpec for PLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfg::R](R) reader structure"]
impl crate::Readable for PLLCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfg::W](W) writer structure"]
impl crate::Writable for PLLCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pllcfg to value 0x0003_06f9"]
impl crate::Resettable for PLLCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_06f9
    }
}
