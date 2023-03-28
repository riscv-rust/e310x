#[doc = "Register `wdogcfg` reader"]
pub struct R(crate::R<WDOGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdogcfg` writer"]
pub struct W(crate::W<WDOGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGCFG_SPEC>;
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
impl From<crate::W<WDOGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `scale` reader - "]
pub type SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `scale` writer - "]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDOGCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `rsten` reader - "]
pub type RSTEN_R = crate::BitReader<bool>;
#[doc = "Field `rsten` writer - "]
pub type RSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOGCFG_SPEC, bool, O>;
#[doc = "Field `zerocmp` reader - "]
pub type ZEROCMP_R = crate::BitReader<bool>;
#[doc = "Field `zerocmp` writer - "]
pub type ZEROCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOGCFG_SPEC, bool, O>;
#[doc = "Field `enalways` reader - "]
pub type ENALWAYS_R = crate::BitReader<bool>;
#[doc = "Field `enalways` writer - "]
pub type ENALWAYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOGCFG_SPEC, bool, O>;
#[doc = "Field `encoreawake` reader - "]
pub type ENCOREAWAKE_R = crate::BitReader<bool>;
#[doc = "Field `encoreawake` writer - "]
pub type ENCOREAWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOGCFG_SPEC, bool, O>;
#[doc = "Field `cmpip` reader - "]
pub type CMPIP_R = crate::BitReader<bool>;
#[doc = "Field `cmpip` writer - "]
pub type CMPIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDOGCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZEROCMP_R {
        ZEROCMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn encoreawake(&self) -> ENCOREAWAKE_R {
        ENCOREAWAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&self) -> CMPIP_R {
        CMPIP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<0> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W<8> {
        RSTEN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&mut self) -> ZEROCMP_W<9> {
        ZEROCMP_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W<12> {
        ENALWAYS_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn encoreawake(&mut self) -> ENCOREAWAKE_W<13> {
        ENCOREAWAKE_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmpip(&mut self) -> CMPIP_W<28> {
        CMPIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogcfg](index.html) module"]
pub struct WDOGCFG_SPEC;
impl crate::RegisterSpec for WDOGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogcfg::R](R) reader structure"]
impl crate::Readable for WDOGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogcfg::W](W) writer structure"]
impl crate::Writable for WDOGCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdogcfg to value 0"]
impl crate::Resettable for WDOGCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
