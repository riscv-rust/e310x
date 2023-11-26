#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    backup: [BACKUP; 16],
}
impl RegisterBlock {
    #[doc = "0x80..0xc0 - Backup Register"]
    #[inline(always)]
    pub const fn backup(&self, n: usize) -> &BACKUP {
        &self.backup[n]
    }
}
#[doc = "backup (rw) register accessor: Backup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup`]
module"]
pub type BACKUP = crate::Reg<backup::BACKUP_SPEC>;
#[doc = "Backup Register"]
pub mod backup;
