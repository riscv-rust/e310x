#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x70],
    lfrosccfg: LFROSCCFG,
}
impl RegisterBlock {
    #[doc = "0x70 - AON Clock Configuration Register"]
    #[inline(always)]
    pub const fn lfrosccfg(&self) -> &LFROSCCFG {
        &self.lfrosccfg
    }
}
#[doc = "lfrosccfg (rw) register accessor: AON Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrosccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrosccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfrosccfg`]
module"]
pub type LFROSCCFG = crate::Reg<lfrosccfg::LFROSCCFG_SPEC>;
#[doc = "AON Clock Configuration Register"]
pub mod lfrosccfg;
