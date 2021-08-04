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
#[doc = "Field `lock` reader - "]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lock` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `bypass` reader - "]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bypass` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `refsel` reader - "]
pub struct REFSEL_R(crate::FieldReader<bool, bool>);
impl REFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `refsel` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `sel` reader - "]
pub struct SEL_R(crate::FieldReader<bool, bool>);
impl SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sel` writer - "]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `pllq` reader - "]
pub struct PLLQ_R(crate::FieldReader<u8, PLLQ_A>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLLQ_A::Q2
    }
    #[doc = "Checks if the value of the field is `Q4`"]
    #[inline(always)]
    pub fn is_q4(&self) -> bool {
        **self == PLLQ_A::Q4
    }
    #[doc = "Checks if the value of the field is `Q8`"]
    #[inline(always)]
    pub fn is_q8(&self) -> bool {
        **self == PLLQ_A::Q8
    }
}
impl core::ops::Deref for PLLQ_R {
    type Target = crate::FieldReader<u8, PLLQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllq` writer - "]
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `pllf` reader - "]
pub struct PLLF_R(crate::FieldReader<u8, u8>);
impl PLLF_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllf` writer - "]
pub struct PLLF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `pllr` reader - "]
pub struct PLLR_R(crate::FieldReader<u8, PLLR_A>);
impl PLLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLLR_A::R1
    }
    #[doc = "Checks if the value of the field is `R2`"]
    #[inline(always)]
    pub fn is_r2(&self) -> bool {
        **self == PLLR_A::R2
    }
    #[doc = "Checks if the value of the field is `R3`"]
    #[inline(always)]
    pub fn is_r3(&self) -> bool {
        **self == PLLR_A::R3
    }
    #[doc = "Checks if the value of the field is `R4`"]
    #[inline(always)]
    pub fn is_r4(&self) -> bool {
        **self == PLLR_A::R4
    }
}
impl core::ops::Deref for PLLR_R {
    type Target = crate::FieldReader<u8, PLLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllr` writer - "]
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
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
