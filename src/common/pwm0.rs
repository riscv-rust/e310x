#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Counter Register"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Scaled Halfword Counter Register"]
    pub pwms: crate::Reg<pwms::PWMS_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x20 - Compare Register"]
    pub cmp0: crate::Reg<cmp0::CMP0_SPEC>,
    #[doc = "0x24 - Compare Register"]
    pub cmp1: crate::Reg<cmp1::CMP1_SPEC>,
    #[doc = "0x28 - Compare Register"]
    pub cmp2: crate::Reg<cmp2::CMP2_SPEC>,
    #[doc = "0x2c - Compare Register"]
    pub cmp3: crate::Reg<cmp3::CMP3_SPEC>,
}
#[doc = "cfg register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "PWM Configuration Register"]
pub mod cfg;
#[doc = "count register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Counter Register"]
pub mod count;
#[doc = "pwms register accessor: an alias for `Reg<PWMS_SPEC>`"]
pub type PWMS = crate::Reg<pwms::PWMS_SPEC>;
#[doc = "Scaled Halfword Counter Register"]
pub mod pwms;
#[doc = "cmp0 register accessor: an alias for `Reg<CMP0_SPEC>`"]
pub type CMP0 = crate::Reg<cmp0::CMP0_SPEC>;
#[doc = "Compare Register"]
pub mod cmp0;
#[doc = "cmp1 register accessor: an alias for `Reg<CMP1_SPEC>`"]
pub type CMP1 = crate::Reg<cmp1::CMP1_SPEC>;
#[doc = "Compare Register"]
pub mod cmp1;
#[doc = "cmp2 register accessor: an alias for `Reg<CMP2_SPEC>`"]
pub type CMP2 = crate::Reg<cmp2::CMP2_SPEC>;
#[doc = "Compare Register"]
pub mod cmp2;
#[doc = "cmp3 register accessor: an alias for `Reg<CMP3_SPEC>`"]
pub type CMP3 = crate::Reg<cmp3::CMP3_SPEC>;
#[doc = "Compare Register"]
pub mod cmp3;
