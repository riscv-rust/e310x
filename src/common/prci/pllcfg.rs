#[doc = "Reader of register pllcfg"]
pub type R = crate::R<u32, super::PLLCFG>;
#[doc = "Writer for register pllcfg"]
pub type W = crate::W<u32, super::PLLCFG>;
#[doc = "Register pllcfg `reset()`'s with value 0x0003_06f9"]
impl crate::ResetValue for super::PLLCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_06f9
    }
}
#[doc = "Reader of field `lock`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lock`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `bypass`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `refsel`"]
pub type REFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `refsel`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `sel`"]
pub type SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLQ_A {
    #[doc = "1: `1`"]
    Q2,
    #[doc = "2: `10`"]
    Q4,
    #[doc = "3: `11`"]
    Q8,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        match variant {
            PLLQ_A::Q2 => 1,
            PLLQ_A::Q4 => 2,
            PLLQ_A::Q8 => 3,
        }
    }
}
#[doc = "Reader of field `pllq`"]
pub type PLLQ_R = crate::R<u8, PLLQ_A>;
impl PLLQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLLQ_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PLLQ_A::Q2),
            2 => Val(PLLQ_A::Q4),
            3 => Val(PLLQ_A::Q8),
            i => Res(i),
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
#[doc = "Write proxy for field `pllq`"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `pllf`"]
pub type PLLF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pllf`"]
pub struct PLLF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLR_A {
    #[doc = "0: `0`"]
    R1,
    #[doc = "1: `1`"]
    R2,
    #[doc = "2: `10`"]
    R3,
    #[doc = "3: `11`"]
    R4,
}
impl From<PLLR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLR_A) -> Self {
        match variant {
            PLLR_A::R1 => 0,
            PLLR_A::R2 => 1,
            PLLR_A::R3 => 2,
            PLLR_A::R4 => 3,
        }
    }
}
#[doc = "Reader of field `pllr`"]
pub type PLLR_R = crate::R<u8, PLLR_A>;
impl PLLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLLR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLLR_A::R1),
            1 => Val(PLLR_A::R2),
            2 => Val(PLLR_A::R3),
            3 => Val(PLLR_A::R4),
            i => Res(i),
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
#[doc = "Write proxy for field `pllr`"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn pllf(&self) -> PLLF_R {
        PLLF_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn pllf(&mut self) -> PLLF_W {
        PLLF_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
}
