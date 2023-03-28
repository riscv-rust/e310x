#[doc = "Register `sckmode` reader"]
pub struct R(crate::R<SCKMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCKMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCKMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCKMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sckmode` writer"]
pub struct W(crate::W<SCKMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCKMODE_SPEC>;
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
impl From<crate::W<SCKMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCKMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pha` reader - Serial clock phase"]
pub type PHA_R = crate::BitReader<bool>;
#[doc = "Field `pha` writer - Serial clock phase"]
pub type PHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCKMODE_SPEC, bool, O>;
#[doc = "Field `pol` reader - Serial clock polarity"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `pol` writer - Serial clock polarity"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCKMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&mut self) -> PHA_W<0> {
        PHA_W::new(self)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<1> {
        POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sckmode](index.html) module"]
pub struct SCKMODE_SPEC;
impl crate::RegisterSpec for SCKMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sckmode::R](R) reader structure"]
impl crate::Readable for SCKMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sckmode::W](W) writer structure"]
impl crate::Writable for SCKMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sckmode to value 0"]
impl crate::Resettable for SCKMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
