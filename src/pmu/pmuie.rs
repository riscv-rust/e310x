#[doc = "Register `pmuie` reader"]
pub struct R(crate::R<PMUIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pmuie` writer"]
pub struct W(crate::W<PMUIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUIE_SPEC>;
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
impl From<crate::W<PMUIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc` reader - "]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `rtc` writer - "]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUIE_SPEC, bool, O>;
#[doc = "Field `dwakeup` reader - "]
pub type DWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `dwakeup` writer - "]
pub type DWAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUIE_SPEC, bool, O>;
#[doc = "Field `awakeup` reader - "]
pub type AWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `awakeup` writer - "]
pub type AWAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&self) -> DWAKEUP_R {
        DWAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&self) -> AWAKEUP_R {
        AWAKEUP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<1> {
        RTC_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&mut self) -> DWAKEUP_W<2> {
        DWAKEUP_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&mut self) -> AWAKEUP_W<3> {
        AWAKEUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmuie](index.html) module"]
pub struct PMUIE_SPEC;
impl crate::RegisterSpec for PMUIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmuie::R](R) reader structure"]
impl crate::Readable for PMUIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmuie::W](W) writer structure"]
impl crate::Writable for PMUIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pmuie to value 0"]
impl crate::Resettable for PMUIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
