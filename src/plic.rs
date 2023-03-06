#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xd4 - Interrupt Priority Register"]
    pub priority: [PRIORITY; 53],
    _reserved1: [u8; 0x0f2c],
    #[doc = "0x1000..0x1008 - Interrupt Pending Register"]
    pub pending: [PENDING; 2],
    _reserved2: [u8; 0x0ff8],
    #[doc = "0x2000..0x2008 - Interrupt Enable Register"]
    pub enable: [ENABLE; 2],
    _reserved3: [u8; 0x001f_dff8],
    #[doc = "0x200000 - Priority Threshold Register"]
    pub threshold: THRESHOLD,
    #[doc = "0x200004 - Claim/Complete Register"]
    pub claim: CLAIM,
}
#[doc = "priority (rw) register accessor: an alias for `Reg<PRIORITY_SPEC>`"]
pub type PRIORITY = crate::Reg<priority::PRIORITY_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod priority;
#[doc = "pending (rw) register accessor: an alias for `Reg<PENDING_SPEC>`"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod pending;
#[doc = "enable (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod enable;
#[doc = "threshold (rw) register accessor: an alias for `Reg<THRESHOLD_SPEC>`"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "claim (rw) register accessor: an alias for `Reg<CLAIM_SPEC>`"]
pub type CLAIM = crate::Reg<claim::CLAIM_SPEC>;
#[doc = "Claim/Complete Register"]
pub mod claim;
