#[doc = "Register `backup[%s]` reader"]
pub type R = crate::R<BackupSpec>;
#[doc = "Register `backup[%s]` writer"]
pub type W = crate::W<BackupSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Backup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BackupSpec;
impl crate::RegisterSpec for BackupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup::R`](R) reader structure"]
impl crate::Readable for BackupSpec {}
#[doc = "`write(|w| ..)` method takes [`backup::W`](W) writer structure"]
impl crate::Writable for BackupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets backup[%s]
to value 0"]
impl crate::Resettable for BackupSpec {
    const RESET_VALUE: u32 = 0;
}
