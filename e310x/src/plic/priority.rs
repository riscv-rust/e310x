#[doc = "Register `priority[%s]` reader"]
pub type R = crate::R<PrioritySpec>;
#[doc = "Register `priority[%s]` writer"]
pub type W = crate::W<PrioritySpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
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
impl From<Priority> for u8 {
    #[inline(always)]
    fn from(variant: Priority) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Priority {
    type Ux = u8;
}
impl crate::IsEnum for Priority {}
#[doc = "Field `priority` reader - "]
pub type PriorityR = crate::FieldReader<Priority>;
impl PriorityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Priority {
        match self.bits {
            0 => Priority::P0,
            1 => Priority::P1,
            2 => Priority::P2,
            3 => Priority::P3,
            4 => Priority::P4,
            5 => Priority::P5,
            6 => Priority::P6,
            7 => Priority::P7,
            _ => unreachable!(),
        }
    }
    #[doc = "Priority 0 (never interrupt)"]
    #[inline(always)]
    pub fn is_p0(&self) -> bool {
        *self == Priority::P0
    }
    #[doc = "Priority 1"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == Priority::P1
    }
    #[doc = "Priority 2"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == Priority::P2
    }
    #[doc = "Priority 3"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == Priority::P3
    }
    #[doc = "Priority 4"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == Priority::P4
    }
    #[doc = "Priority 5"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == Priority::P5
    }
    #[doc = "Priority 6"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        *self == Priority::P6
    }
    #[doc = "Priority 7"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == Priority::P7
    }
}
#[doc = "Field `priority` writer - "]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3, Priority, crate::Safe>;
impl<'a, REG> PriorityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Priority 0 (never interrupt)"]
    #[inline(always)]
    pub fn p0(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P0)
    }
    #[doc = "Priority 1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P1)
    }
    #[doc = "Priority 2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P2)
    }
    #[doc = "Priority 3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P3)
    }
    #[doc = "Priority 4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P4)
    }
    #[doc = "Priority 5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P5)
    }
    #[doc = "Priority 6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P6)
    }
    #[doc = "Priority 7"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut crate::W<REG> {
        self.variant(Priority::P7)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<PrioritySpec> {
        PriorityW::new(self, 0)
    }
}
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioritySpec;
impl crate::RegisterSpec for PrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PrioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets priority[%s]
to value 0"]
impl crate::Resettable for PrioritySpec {
    const RESET_VALUE: u32 = 0;
}
