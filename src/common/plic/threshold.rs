#[doc = "Register `threshold` reader"]
pub struct R(crate::R<THRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `threshold` writer"]
pub struct W(crate::W<THRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESHOLD_SPEC>;
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
impl From<crate::W<THRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRIORITY_A {
    #[doc = "0: Allow interrupts with priority > 0"]
    P0 = 0,
    #[doc = "1: Allow interrupts with priority > 1"]
    P1 = 1,
    #[doc = "2: Allow interrupts with priority > 2"]
    P2 = 2,
    #[doc = "3: Allow interrupts with priority > 3"]
    P3 = 3,
    #[doc = "4: Allow interrupts with priority > 4"]
    P4 = 4,
    #[doc = "5: Allow interrupts with priority > 5"]
    P5 = 5,
    #[doc = "6: Allow interrupts with priority > 6"]
    P6 = 6,
    #[doc = "7: Mask all interrupts"]
    P7 = 7,
}
impl From<PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIORITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `priority` reader - "]
pub struct PRIORITY_R(crate::FieldReader<u8, PRIORITY_A>);
impl PRIORITY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIORITY_A {
        match self.bits {
            0 => PRIORITY_A::P0,
            1 => PRIORITY_A::P1,
            2 => PRIORITY_A::P2,
            3 => PRIORITY_A::P3,
            4 => PRIORITY_A::P4,
            5 => PRIORITY_A::P5,
            6 => PRIORITY_A::P6,
            7 => PRIORITY_A::P7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P0`"]
    #[inline(always)]
    pub fn is_p0(&self) -> bool {
        **self == PRIORITY_A::P0
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        **self == PRIORITY_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        **self == PRIORITY_A::P2
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        **self == PRIORITY_A::P3
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        **self == PRIORITY_A::P4
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        **self == PRIORITY_A::P5
    }
    #[doc = "Checks if the value of the field is `P6`"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        **self == PRIORITY_A::P6
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        **self == PRIORITY_A::P7
    }
}
impl core::ops::Deref for PRIORITY_R {
    type Target = crate::FieldReader<u8, PRIORITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `priority` writer - "]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIORITY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Allow interrupts with priority > 0"]
    #[inline(always)]
    pub fn p0(self) -> &'a mut W {
        self.variant(PRIORITY_A::P0)
    }
    #[doc = "Allow interrupts with priority > 1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(PRIORITY_A::P1)
    }
    #[doc = "Allow interrupts with priority > 2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(PRIORITY_A::P2)
    }
    #[doc = "Allow interrupts with priority > 3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut W {
        self.variant(PRIORITY_A::P3)
    }
    #[doc = "Allow interrupts with priority > 4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(PRIORITY_A::P4)
    }
    #[doc = "Allow interrupts with priority > 5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut W {
        self.variant(PRIORITY_A::P5)
    }
    #[doc = "Allow interrupts with priority > 6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut W {
        self.variant(PRIORITY_A::P6)
    }
    #[doc = "Mask all interrupts"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut W {
        self.variant(PRIORITY_A::P7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold](index.html) module"]
pub struct THRESHOLD_SPEC;
impl crate::RegisterSpec for THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [threshold::R](R) reader structure"]
impl crate::Readable for THRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [threshold::W](W) writer structure"]
impl crate::Writable for THRESHOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets threshold to value 0"]
impl crate::Resettable for THRESHOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
