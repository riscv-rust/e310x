#[doc = "Register `txctrl` reader"]
pub type R = crate::R<TxctrlSpec>;
#[doc = "Register `txctrl` writer"]
pub type W = crate::W<TxctrlSpec>;
#[doc = "Field `enable` reader - "]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - "]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nstop` reader - "]
pub type NstopR = crate::BitReader;
#[doc = "Field `nstop` writer - "]
pub type NstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `counter` reader - "]
pub type CounterR = crate::FieldReader;
#[doc = "Field `counter` writer - "]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&self) -> NstopR {
        NstopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<TxctrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nstop(&mut self) -> NstopW<TxctrlSpec> {
        NstopW::new(self, 1)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn counter(&mut self) -> CounterW<TxctrlSpec> {
        CounterW::new(self, 16)
    }
}
#[doc = "Transmit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctrlSpec;
impl crate::RegisterSpec for TxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txctrl to value 0"]
impl crate::Resettable for TxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
