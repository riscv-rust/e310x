#[doc = "Register `csmode` reader"]
pub type R = crate::R<CsmodeSpec>;
#[doc = "Register `csmode` writer"]
pub type W = crate::W<CsmodeSpec>;
#[doc = "Chip select mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Assert/de-assert CS at the beginning/end of each frame."]
    Auto = 0,
    #[doc = "2: Keep CS continuously asserted after the initial frame."]
    Hold = 2,
    #[doc = "3: Disable hardware control of the CS pin."]
    Off = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `mode` reader - Chip select mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Auto),
            2 => Some(Mode::Hold),
            3 => Some(Mode::Off),
            _ => None,
        }
    }
    #[doc = "Assert/de-assert CS at the beginning/end of each frame."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Mode::Auto
    }
    #[doc = "Keep CS continuously asserted after the initial frame."]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == Mode::Hold
    }
    #[doc = "Disable hardware control of the CS pin."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
}
#[doc = "Field `mode` writer - Chip select mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Assert/de-assert CS at the beginning/end of each frame."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Auto)
    }
    #[doc = "Keep CS continuously asserted after the initial frame."]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Hold)
    }
    #[doc = "Disable hardware control of the CS pin."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
}
impl R {
    #[doc = "Bits 0:1 - Chip select mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chip select mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CsmodeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Chip Select Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsmodeSpec;
impl crate::RegisterSpec for CsmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csmode::R`](R) reader structure"]
impl crate::Readable for CsmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`csmode::W`](W) writer structure"]
impl crate::Writable for CsmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets csmode to value 0"]
impl crate::Resettable for CsmodeSpec {
    const RESET_VALUE: u32 = 0;
}
