#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - RTC Configuration Register"]
    pub rtccfg: crate::Reg<rtccfg::RTCCFG_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x48 - RTC Counter Low Register"]
    pub rtclo: crate::Reg<rtclo::RTCLO_SPEC>,
    #[doc = "0x4c - RTC Counter High Register"]
    pub rtchi: crate::Reg<rtchi::RTCHI_SPEC>,
    #[doc = "0x50 - RTC Scaled Counter Register"]
    pub rtcs: crate::Reg<rtcs::RTCS_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x60 - RTC Compare Register"]
    pub rtccmp: crate::Reg<rtccmp::RTCCMP_SPEC>,
}
#[doc = "rtccfg register accessor: an alias for `Reg<RTCCFG_SPEC>`"]
pub type RTCCFG = crate::Reg<rtccfg::RTCCFG_SPEC>;
#[doc = "RTC Configuration Register"]
pub mod rtccfg;
#[doc = "rtclo register accessor: an alias for `Reg<RTCLO_SPEC>`"]
pub type RTCLO = crate::Reg<rtclo::RTCLO_SPEC>;
#[doc = "RTC Counter Low Register"]
pub mod rtclo;
#[doc = "rtchi register accessor: an alias for `Reg<RTCHI_SPEC>`"]
pub type RTCHI = crate::Reg<rtchi::RTCHI_SPEC>;
#[doc = "RTC Counter High Register"]
pub mod rtchi;
#[doc = "rtcs register accessor: an alias for `Reg<RTCS_SPEC>`"]
pub type RTCS = crate::Reg<rtcs::RTCS_SPEC>;
#[doc = "RTC Scaled Counter Register"]
pub mod rtcs;
#[doc = "rtccmp register accessor: an alias for `Reg<RTCCMP_SPEC>`"]
pub type RTCCMP = crate::Reg<rtccmp::RTCCMP_SPEC>;
#[doc = "RTC Compare Register"]
pub mod rtccmp;
