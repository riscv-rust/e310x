#[doc = "Reader of register threshold"]
pub type R = crate::R<u32, super::THRESHOLD>;
#[doc = "Writer for register threshold"]
pub type W = crate::W<u32, super::THRESHOLD>;
#[doc = "Register threshold `reset()`'s with value 0"]
impl crate::ResetValue for super::THRESHOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `priority`"]
pub type PRIORITY_R = crate::R<u8, PRIORITY_A>;
impl PRIORITY_R {
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
        *self == PRIORITY_A::P0
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == PRIORITY_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == PRIORITY_A::P2
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == PRIORITY_A::P3
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == PRIORITY_A::P4
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == PRIORITY_A::P5
    }
    #[doc = "Checks if the value of the field is `P6`"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        *self == PRIORITY_A::P6
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == PRIORITY_A::P7
    }
}
#[doc = "Write proxy for field `priority`"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIORITY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
}
