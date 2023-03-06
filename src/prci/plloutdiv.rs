#[doc = "Register `plloutdiv` reader"]
pub struct R(crate::R<PLLOUTDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLOUTDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLOUTDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLOUTDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `plloutdiv` writer"]
pub struct W(crate::W<PLLOUTDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLOUTDIV_SPEC>;
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
impl From<crate::W<PLLOUTDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLOUTDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `div` reader - "]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `div` writer - "]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLOUTDIV_SPEC, u8, u8, 6, O>;
#[doc = "Field `divby1` reader - "]
pub type DIVBY1_R = crate::BitReader<bool>;
#[doc = "Field `divby1` writer - "]
pub type DIVBY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLOUTDIV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&self) -> DIVBY1_R {
        DIVBY1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&mut self) -> DIVBY1_W<8> {
        DIVBY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plloutdiv](index.html) module"]
pub struct PLLOUTDIV_SPEC;
impl crate::RegisterSpec for PLLOUTDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plloutdiv::R](R) reader structure"]
impl crate::Readable for PLLOUTDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plloutdiv::W](W) writer structure"]
impl crate::Writable for PLLOUTDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets plloutdiv to value 0x0100"]
impl crate::Resettable for PLLOUTDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
