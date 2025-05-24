#[doc = "Register `rsctrl` reader"]
pub type R = crate::R<RsctrlSpec>;
#[doc = "Register `rsctrl` writer"]
pub type W = crate::W<RsctrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP read sequencer control\n\nYou can [`read`](crate::Reg::read) this register and get [`rsctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsctrlSpec;
impl crate::RegisterSpec for RsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsctrl::R`](R) reader structure"]
impl crate::Readable for RsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rsctrl::W`](W) writer structure"]
impl crate::Writable for RsctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rsctrl to value 0"]
impl crate::Resettable for RsctrlSpec {}
