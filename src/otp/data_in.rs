#[doc = "Register `data_in` reader"]
pub type R = crate::R<DataInSpec>;
#[doc = "Register `data_in` writer"]
pub type W = crate::W<DataInSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OTP device data input\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataInSpec;
impl crate::RegisterSpec for DataInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_in::R`](R) reader structure"]
impl crate::Readable for DataInSpec {}
#[doc = "`write(|w| ..)` method takes [`data_in::W`](W) writer structure"]
impl crate::Writable for DataInSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets data_in to value 0"]
impl crate::Resettable for DataInSpec {
    const RESET_VALUE: u32 = 0;
}
