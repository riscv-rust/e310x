#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80..0xc0 - Backup Register"]
    pub backup: [BACKUP; 16],
}
#[doc = "backup (rw) register accessor: an alias for `Reg<BACKUP_SPEC>`"]
pub type BACKUP = crate::Reg<backup::BACKUP_SPEC>;
#[doc = "Backup Register"]
pub mod backup;
