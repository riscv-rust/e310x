#[doc = "Register `ip` reader"]
pub type R = crate::R<IpSpec>;
#[doc = "Register `ip` writer"]
pub type W = crate::W<IpSpec>;
#[doc = "Field `txwm` reader - "]
pub type TxwmR = crate::BitReader;
#[doc = "Field `txwm` writer - "]
pub type TxwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxwm` reader - "]
pub type RxwmR = crate::BitReader;
#[doc = "Field `rxwm` writer - "]
pub type RxwmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txwm(&self) -> TxwmR {
        TxwmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxwm(&self) -> RxwmR {
        RxwmR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txwm(&mut self) -> TxwmW<IpSpec> {
        TxwmW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxwm(&mut self) -> RxwmW<IpSpec> {
        RxwmW::new(self, 1)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpSpec;
impl crate::RegisterSpec for IpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip::R`](R) reader structure"]
impl crate::Readable for IpSpec {}
#[doc = "`write(|w| ..)` method takes [`ip::W`](W) writer structure"]
impl crate::Writable for IpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ip to value 0"]
impl crate::Resettable for IpSpec {
    const RESET_VALUE: u32 = 0;
}
