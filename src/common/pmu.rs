#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100..0x120 - PMU Wake Program Memory"]
    pub pmuwakepm: [crate::Reg<pmuwakepm::PMUWAKEPM_SPEC>; 8],
    #[doc = "0x120..0x140 - PMU Sleep Program Memory"]
    pub pmusleeppm: [crate::Reg<pmusleeppm::PMUSLEEPPM_SPEC>; 8],
    #[doc = "0x140 - PMU Interrupt Enable Register"]
    pub pmuie: crate::Reg<pmuie::PMUIE_SPEC>,
    #[doc = "0x144 - PMU Cause Register"]
    pub pmucause: crate::Reg<pmucause::PMUCAUSE_SPEC>,
    #[doc = "0x148 - PMU Sleep Register"]
    pub pmusleep: crate::Reg<pmusleep::PMUSLEEP_SPEC>,
    #[doc = "0x14c - PMU Key Register"]
    pub pmukey: crate::Reg<pmukey::PMUKEY_SPEC>,
}
#[doc = "pmuwakepm register accessor: an alias for `Reg<PMUWAKEPM_SPEC>`"]
pub type PMUWAKEPM = crate::Reg<pmuwakepm::PMUWAKEPM_SPEC>;
#[doc = "PMU Wake Program Memory"]
pub mod pmuwakepm;
#[doc = "pmusleeppm register accessor: an alias for `Reg<PMUSLEEPPM_SPEC>`"]
pub type PMUSLEEPPM = crate::Reg<pmusleeppm::PMUSLEEPPM_SPEC>;
#[doc = "PMU Sleep Program Memory"]
pub mod pmusleeppm;
#[doc = "pmuie register accessor: an alias for `Reg<PMUIE_SPEC>`"]
pub type PMUIE = crate::Reg<pmuie::PMUIE_SPEC>;
#[doc = "PMU Interrupt Enable Register"]
pub mod pmuie;
#[doc = "pmucause register accessor: an alias for `Reg<PMUCAUSE_SPEC>`"]
pub type PMUCAUSE = crate::Reg<pmucause::PMUCAUSE_SPEC>;
#[doc = "PMU Cause Register"]
pub mod pmucause;
#[doc = "pmusleep register accessor: an alias for `Reg<PMUSLEEP_SPEC>`"]
pub type PMUSLEEP = crate::Reg<pmusleep::PMUSLEEP_SPEC>;
#[doc = "PMU Sleep Register"]
pub mod pmusleep;
#[doc = "pmukey register accessor: an alias for `Reg<PMUKEY_SPEC>`"]
pub type PMUKEY = crate::Reg<pmukey::PMUKEY_SPEC>;
#[doc = "PMU Key Register"]
pub mod pmukey;
