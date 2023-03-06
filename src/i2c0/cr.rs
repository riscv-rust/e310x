#[doc = "Register `cr` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `iack` writer - Interrupt acknowledge. When set, clears a pending interrupt"]
pub type IACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "When a receiver, sent ACK (0) or NACK (1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACK_AW {
    #[doc = "0: `0`"]
    ACK = 0,
    #[doc = "1: `1`"]
    NACK = 1,
}
impl From<ACK_AW> for bool {
    #[inline(always)]
    fn from(variant: ACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ack` writer - When a receiver, sent ACK (0) or NACK (1)"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ACK_AW, O>;
impl<'a, const O: u8> ACK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(ACK_AW::ACK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(ACK_AW::NACK)
    }
}
#[doc = "Field `wr` writer - Write to slave"]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `rd` writer - Read from slave"]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `sto` writer - Generate stop condition"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `sta` writer - Generate (repeated) start condition"]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Interrupt acknowledge. When set, clears a pending interrupt"]
    #[inline(always)]
    pub fn iack(&mut self) -> IACK_W<0> {
        IACK_W::new(self)
    }
    #[doc = "Bit 3 - When a receiver, sent ACK (0) or NACK (1)"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<3> {
        ACK_W::new(self)
    }
    #[doc = "Bit 4 - Write to slave"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W<4> {
        WR_W::new(self)
    }
    #[doc = "Bit 5 - Read from slave"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W<5> {
        RD_W::new(self)
    }
    #[doc = "Bit 6 - Generate stop condition"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W<6> {
        STO_W::new(self)
    }
    #[doc = "Bit 7 - Generate (repeated) start condition"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W<7> {
        STA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
