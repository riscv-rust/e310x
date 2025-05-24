#[doc = "Register `data_out` reader"]
pub type R = crate::R<DataOutSpec>;
#[doc = "Register `data_out` writer"]
pub type W = crate::W<DataOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP device data output\n\nYou can [`read`](crate::Reg::read) this register and get [`data_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataOutSpec;
impl crate::RegisterSpec for DataOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_out::R`](R) reader structure"]
impl crate::Readable for DataOutSpec {}
#[doc = "`write(|w| ..)` method takes [`data_out::W`](W) writer structure"]
impl crate::Writable for DataOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets data_out to value 0"]
impl crate::Resettable for DataOutSpec {}
