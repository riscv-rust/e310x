#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Priority Register"]
    pub priority: [PRIORITY; 52],
    _reserved0: [u8; 3888usize],
    #[doc = "0x1000 - Interrupt Pending Register"]
    pub pending: [PENDING; 2],
    _reserved1: [u8; 4088usize],
    #[doc = "0x2000 - Interrupt Enable Register"]
    pub enable: [ENABLE; 2],
    _reserved2: [u8; 2088952usize],
    #[doc = "0x200000 - Priority Threshold Register"]
    pub threshold: THRESHOLD,
    #[doc = "0x200004 - Claim/Complete Register"]
    pub claim: CLAIM,
}
#[doc = "Interrupt Priority Register"]
pub struct PRIORITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register"]
pub mod priority;
#[doc = "Interrupt Pending Register"]
pub struct PENDING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending Register"]
pub mod pending;
#[doc = "Interrupt Enable Register"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod enable;
#[doc = "Priority Threshold Register"]
pub struct THRESHOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "Claim/Complete Register"]
pub struct CLAIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim/Complete Register"]
pub mod claim;
