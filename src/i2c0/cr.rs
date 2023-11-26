#[doc = "Register `cr` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `iack` writer - Interrupt acknowledge. When set, clears a pending interrupt"]
pub type IACK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG, ACK_AW>;
impl<'a, REG> ACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ACK_AW::ACK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(ACK_AW::NACK)
    }
}
#[doc = "Field `wr` writer - Write to slave"]
pub type WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd` writer - Read from slave"]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sto` writer - Generate stop condition"]
pub type STO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sta` writer - Generate (repeated) start condition"]
pub type STA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt acknowledge. When set, clears a pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn iack(&mut self) -> IACK_W<CR_SPEC> {
        IACK_W::new(self, 0)
    }
    #[doc = "Bit 3 - When a receiver, sent ACK (0) or NACK (1)"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CR_SPEC> {
        ACK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write to slave"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<CR_SPEC> {
        WR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Read from slave"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<CR_SPEC> {
        RD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Generate stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<CR_SPEC> {
        STO_W::new(self, 6)
    }
    #[doc = "Bit 7 - Generate (repeated) start condition"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<CR_SPEC> {
        STA_W::new(self, 7)
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
#[doc = "Command register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
