#[doc = "Register `cr` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `iack` writer - Interrupt acknowledge. When set, clears a pending interrupt"]
pub type IackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "When a receiver, sent ACK (0) or NACK (1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    #[doc = "0: `0`"]
    Ack = 0,
    #[doc = "1: `1`"]
    Nack = 1,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ack` writer - When a receiver, sent ACK (0) or NACK (1)"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG, Ack>;
impl<'a, REG> AckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Ack)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(Ack::Nack)
    }
}
#[doc = "Field `wr` writer - Write to slave"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd` writer - Read from slave"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sto` writer - Generate stop condition"]
pub type StoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sta` writer - Generate (repeated) start condition"]
pub type StaW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt acknowledge. When set, clears a pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn iack(&mut self) -> IackW<CrSpec> {
        IackW::new(self, 0)
    }
    #[doc = "Bit 3 - When a receiver, sent ACK (0) or NACK (1)"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<CrSpec> {
        AckW::new(self, 3)
    }
    #[doc = "Bit 4 - Write to slave"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<CrSpec> {
        WrW::new(self, 4)
    }
    #[doc = "Bit 5 - Read from slave"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<CrSpec> {
        RdW::new(self, 5)
    }
    #[doc = "Bit 6 - Generate stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> StoW<CrSpec> {
        StoW::new(self, 6)
    }
    #[doc = "Bit 7 - Generate (repeated) start condition"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> StaW<CrSpec> {
        StaW::new(self, 7)
    }
}
#[doc = "Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
