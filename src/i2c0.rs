#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Prescale register lo-byte"]
    pub prer_lo: PRER_LO,
    #[doc = "0x04 - Clock Prescale register hi-byte"]
    pub prer_hi: PRER_HI,
    #[doc = "0x08 - Control register"]
    pub ctr: CTR,
    #[doc = "0x0c - Transmit register / Receive register"]
    pub txr_rxr: TXR_RXR,
    _reserved_4_cr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub fn sr(&self) -> &SR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const SR) }
    }
    #[doc = "0x10 - Command register"]
    #[inline(always)]
    pub fn cr(&self) -> &CR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const CR) }
    }
    #[doc = "0x10 - Command register / Status register"]
    #[inline(always)]
    pub fn cr_sr(&self) -> &CR_SR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const CR_SR) }
    }
}
#[doc = "prer_lo (rw) register accessor: an alias for `Reg<PRER_LO_SPEC>`"]
pub type PRER_LO = crate::Reg<prer_lo::PRER_LO_SPEC>;
#[doc = "Clock Prescale register lo-byte"]
pub mod prer_lo;
#[doc = "prer_hi (rw) register accessor: an alias for `Reg<PRER_HI_SPEC>`"]
pub type PRER_HI = crate::Reg<prer_hi::PRER_HI_SPEC>;
#[doc = "Clock Prescale register hi-byte"]
pub mod prer_hi;
#[doc = "ctr (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Control register"]
pub mod ctr;
#[doc = "txr_rxr (rw) register accessor: an alias for `Reg<TXR_RXR_SPEC>`"]
pub type TXR_RXR = crate::Reg<txr_rxr::TXR_RXR_SPEC>;
#[doc = "Transmit register / Receive register"]
pub mod txr_rxr;
#[doc = "cr_sr (rw) register accessor: an alias for `Reg<CR_SR_SPEC>`"]
pub type CR_SR = crate::Reg<cr_sr::CR_SR_SPEC>;
#[doc = "Command register / Status register"]
pub mod cr_sr;
#[doc = "cr (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Command register"]
pub mod cr;
#[doc = "sr (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
