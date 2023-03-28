#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hart 0 software interrupt register"]
    pub msip: MSIP,
    _reserved1: [u8; 0x3ffc],
    #[doc = "0x4000 - Hart 0 time comparator register"]
    pub mtimecmp: MTIMECMP,
    #[doc = "0x4004 - Hart 0 time comparator register"]
    pub mtimecmph: MTIMECMPH,
    _reserved3: [u8; 0x7ff0],
    #[doc = "0xbff8 - Timer register"]
    pub mtime: MTIME,
    #[doc = "0xbffc - Timer register"]
    pub mtimeh: MTIMEH,
}
#[doc = "msip (rw) register accessor: an alias for `Reg<MSIP_SPEC>`"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Hart 0 software interrupt register"]
pub mod msip;
#[doc = "mtimecmp (rw) register accessor: an alias for `Reg<MTIMECMP_SPEC>`"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmp;
#[doc = "mtimecmph (rw) register accessor: an alias for `Reg<MTIMECMPH_SPEC>`"]
pub type MTIMECMPH = crate::Reg<mtimecmph::MTIMECMPH_SPEC>;
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmph;
#[doc = "mtime (rw) register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Timer register"]
pub mod mtime;
#[doc = "mtimeh (rw) register accessor: an alias for `Reg<MTIMEH_SPEC>`"]
pub type MTIMEH = crate::Reg<mtimeh::MTIMEH_SPEC>;
#[doc = "Timer register"]
pub mod mtimeh;
