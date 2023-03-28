#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x70],
    #[doc = "0x70 - AON Clock Configuration Register"]
    pub lfrosccfg: LFROSCCFG,
}
#[doc = "lfrosccfg (rw) register accessor: an alias for `Reg<LFROSCCFG_SPEC>`"]
pub type LFROSCCFG = crate::Reg<lfrosccfg::LFROSCCFG_SPEC>;
#[doc = "AON Clock Configuration Register"]
pub mod lfrosccfg;
