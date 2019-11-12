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
pub enum PRIORITY_A {
    #[doc = "0: Never interrupt"]
    NEVER,
    #[doc = "1: Priority 1"]
    P1,
    #[doc = "2: Priority 2"]
    P2,
    #[doc = "3: Priority 3"]
    P3,
    #[doc = "4: Priority 4"]
    P4,
    #[doc = "5: Priority 5"]
    P5,
    #[doc = "6: Priority 6"]
    P6,
    #[doc = "7: Priority 7"]
    P7,
}
impl From<PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIORITY_A) -> Self {
        match variant {
            PRIORITY_A::NEVER => 0,
            PRIORITY_A::P1 => 1,
            PRIORITY_A::P2 => 2,
            PRIORITY_A::P3 => 3,
            PRIORITY_A::P4 => 4,
            PRIORITY_A::P5 => 5,
            PRIORITY_A::P6 => 6,
            PRIORITY_A::P7 => 7,
        }
    }
}
#[doc = "Reader of field `priority`"]
pub type PRIORITY_R = crate::R<u8, PRIORITY_A>;
impl PRIORITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIORITY_A {
        match self.bits {
            0 => PRIORITY_A::NEVER,
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
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == PRIORITY_A::NEVER
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
    #[doc = "Never interrupt"]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(PRIORITY_A::NEVER)
    }
    #[doc = "Priority 1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(PRIORITY_A::P1)
    }
    #[doc = "Priority 2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(PRIORITY_A::P2)
    }
    #[doc = "Priority 3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut W {
        self.variant(PRIORITY_A::P3)
    }
    #[doc = "Priority 4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(PRIORITY_A::P4)
    }
    #[doc = "Priority 5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut W {
        self.variant(PRIORITY_A::P5)
    }
    #[doc = "Priority 6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut W {
        self.variant(PRIORITY_A::P6)
    }
    #[doc = "Priority 7"]
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
