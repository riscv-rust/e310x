#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x70],
    lfrosccfg: Lfrosccfg,
}
impl RegisterBlock {
    #[doc = "0x70 - AON Clock Configuration Register"]
    #[inline(always)]
    pub const fn lfrosccfg(&self) -> &Lfrosccfg {
        &self.lfrosccfg
    }
}
#[doc = "lfrosccfg (rw) register accessor: AON Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrosccfg`]
module"]
#[doc(alias = "lfrosccfg")]
pub type Lfrosccfg = crate::Reg<lfrosccfg::LfrosccfgSpec>;
#[doc = "AON Clock Configuration Register"]
pub mod lfrosccfg;
