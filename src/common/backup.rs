#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - Backup Register"]
    pub backup: [BACKUP; 16],
}
#[doc = "Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [backup](backup) module"]
pub type BACKUP = crate::Reg<u32, _BACKUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKUP;
#[doc = "`read()` method returns [backup::R](backup::R) reader structure"]
impl crate::Readable for BACKUP {}
#[doc = "`write(|w| ..)` method takes [backup::W](backup::W) writer structure"]
impl crate::Writable for BACKUP {}
#[doc = "Backup Register"]
pub mod backup;
