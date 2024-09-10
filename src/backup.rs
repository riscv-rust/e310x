#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    backup: [Backup; 16],
}
impl RegisterBlock {
    #[doc = "0x80..0xc0 - Backup Register"]
    #[inline(always)]
    pub const fn backup(&self, n: usize) -> &Backup {
        &self.backup[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Backup Register"]
    #[inline(always)]
    pub fn backup_iter(&self) -> impl Iterator<Item = &Backup> {
        self.backup.iter()
    }
}
#[doc = "backup (rw) register accessor: Backup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`backup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup`]
module"]
#[doc(alias = "backup")]
pub type Backup = crate::Reg<backup::BackupSpec>;
#[doc = "Backup Register"]
pub mod backup;
