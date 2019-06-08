#[doc = r" Register block"]
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
    #[doc = "0x10 - Command register / Status register"]
    pub cr_sr: CR_SR,
}
#[doc = "Clock Prescale register lo-byte"]
pub struct PRER_LO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Prescale register lo-byte"]
pub mod prer_lo;
#[doc = "Clock Prescale register hi-byte"]
pub struct PRER_HI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Prescale register hi-byte"]
pub mod prer_hi;
#[doc = "Control register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctr;
#[doc = "Transmit register / Receive register"]
pub struct TXR_RXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit register / Receive register"]
pub mod txr_rxr;
#[doc = "Command register / Status register"]
pub struct CR_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command register / Status register"]
pub mod cr_sr;
#[doc = "Command register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command register"]
pub mod cr;
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
