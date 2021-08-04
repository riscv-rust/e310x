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
#[doc = "Field `awakeup` reader - "]
pub struct AWAKEUP_R(crate::FieldReader<bool, bool>);
impl AWAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `awakeup` writer - "]
pub struct AWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> AWAKEUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `dwakeup` reader - "]
pub struct DWAKEUP_R(crate::FieldReader<bool, bool>);
impl DWAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DWAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dwakeup` writer - "]
pub struct DWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DWAKEUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `rtc` reader - "]
pub struct RTC_R(crate::FieldReader<bool, bool>);
impl RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc` writer - "]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&self) -> AWAKEUP_R {
        AWAKEUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&self) -> DWAKEUP_R {
        DWAKEUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awakeup(&mut self) -> AWAKEUP_W {
        AWAKEUP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwakeup(&mut self) -> DWAKEUP_W {
        DWAKEUP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
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
