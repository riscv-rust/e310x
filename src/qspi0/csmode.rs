#[doc = "Register `csmode` reader"]
pub type R = crate::R<CSMODE_SPEC>;
#[doc = "Register `csmode` writer"]
pub type W = crate::W<CSMODE_SPEC>;
#[doc = "Field `mode` reader - Chip select mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Chip select mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Assert/de-assert CS at the beginning/end of each frame."]
    AUTO = 0,
    #[doc = "2: Keep CS continuously asserted after the initial frame."]
    HOLD = 2,
    #[doc = "3: Disable hardware control of the CS pin."]
    OFF = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::AUTO),
            2 => Some(MODE_A::HOLD),
            3 => Some(MODE_A::OFF),
            _ => None,
        }
    }
    #[doc = "Assert/de-assert CS at the beginning/end of each frame."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == MODE_A::AUTO
    }
    #[doc = "Keep CS continuously asserted after the initial frame."]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == MODE_A::HOLD
    }
    #[doc = "Disable hardware control of the CS pin."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
}
#[doc = "Field `mode` writer - Chip select mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Assert/de-assert CS at the beginning/end of each frame."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::AUTO)
    }
    #[doc = "Keep CS continuously asserted after the initial frame."]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::HOLD)
    }
    #[doc = "Disable hardware control of the CS pin."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OFF)
    }
}
impl R {
    #[doc = "Bits 0:1 - Chip select mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chip select mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CSMODE_SPEC> {
        MODE_W::new(self, 0)
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
#[doc = "Chip Select Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSMODE_SPEC;
impl crate::RegisterSpec for CSMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csmode::R`](R) reader structure"]
impl crate::Readable for CSMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csmode::W`](W) writer structure"]
impl crate::Writable for CSMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csmode to value 0"]
impl crate::Resettable for CSMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
