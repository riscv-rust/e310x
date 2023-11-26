#[doc = "Register `priority[%s]` reader"]
pub type R = crate::R<PRIORITY_SPEC>;
#[doc = "Register `priority[%s]` writer"]
pub type W = crate::W<PRIORITY_SPEC>;
#[doc = "Field `priority` reader - "]
pub type PRIORITY_R = crate::FieldReader<PRIORITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIORITY_A {
    #[doc = "0: Priority 0 (never interrupt)"]
    P0 = 0,
    #[doc = "1: Priority 1"]
    P1 = 1,
    #[doc = "2: Priority 2"]
    P2 = 2,
    #[doc = "3: Priority 3"]
    P3 = 3,
    #[doc = "4: Priority 4"]
    P4 = 4,
    #[doc = "5: Priority 5"]
    P5 = 5,
    #[doc = "6: Priority 6"]
    P6 = 6,
    #[doc = "7: Priority 7"]
    P7 = 7,
}
impl From<PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIORITY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIORITY_A {
    type Ux = u8;
}
impl PRIORITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIORITY_A {
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
    #[doc = "Priority 0 (never interrupt)"]
    #[inline(always)]
    pub fn is_p0(&self) -> bool {
        *self == PRIORITY_A::P0
    }
    #[doc = "Priority 1"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == PRIORITY_A::P1
    }
    #[doc = "Priority 2"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == PRIORITY_A::P2
    }
    #[doc = "Priority 3"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == PRIORITY_A::P3
    }
    #[doc = "Priority 4"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == PRIORITY_A::P4
    }
    #[doc = "Priority 5"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == PRIORITY_A::P5
    }
    #[doc = "Priority 6"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        *self == PRIORITY_A::P6
    }
    #[doc = "Priority 7"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == PRIORITY_A::P7
    }
}
#[doc = "Field `priority` writer - "]
pub type PRIORITY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PRIORITY_A>;
impl<'a, REG> PRIORITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Priority 0 (never interrupt)"]
    #[inline(always)]
    pub fn p0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P0)
    }
    #[doc = "Priority 1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P1)
    }
    #[doc = "Priority 2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P2)
    }
    #[doc = "Priority 3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P3)
    }
    #[doc = "Priority 4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P4)
    }
    #[doc = "Priority 5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P5)
    }
    #[doc = "Priority 6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P6)
    }
    #[doc = "Priority 7"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut crate::W<REG> {
        self.variant(PRIORITY_A::P7)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<PRIORITY_SPEC> {
        PRIORITY_W::new(self, 0)
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
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIORITY_SPEC;
impl crate::RegisterSpec for PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PRIORITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PRIORITY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets priority[%s]
to value 0"]
impl crate::Resettable for PRIORITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
