#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - RTC Configuration Register"]
    pub rtccfg: RTCCFG,
    _reserved1: [u8; 4usize],
    #[doc = "0x48 - RTC Counter Low Register"]
    pub rtclo: RTCLO,
    #[doc = "0x4c - RTC Counter High Register"]
    pub rtchi: RTCHI,
    #[doc = "0x50 - RTC Scaled Counter Register"]
    pub rtcs: RTCS,
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - RTC Compare Register"]
    pub rtccmp: RTCCMP,
}
#[doc = "RTC Configuration Register"]
pub struct RTCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Configuration Register"]
pub mod rtccfg;
#[doc = "RTC Counter Low Register"]
pub struct RTCLO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Counter Low Register"]
pub mod rtclo;
#[doc = "RTC Counter High Register"]
pub struct RTCHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Counter High Register"]
pub mod rtchi;
#[doc = "RTC Scaled Counter Register"]
pub struct RTCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Scaled Counter Register"]
pub mod rtcs;
#[doc = "RTC Compare Register"]
pub struct RTCCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Compare Register"]
pub mod rtccmp;
