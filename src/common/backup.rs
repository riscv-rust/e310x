#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - Backup Register"]
    pub backup: [BACKUP; 32],
}
#[doc = "Backup Register"]
pub struct BACKUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Register"]
pub mod backup;
