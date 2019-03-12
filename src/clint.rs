#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hart 0 software interrupt register"]
    pub msip: MSIP,
    _reserved0: [u8; 16380usize],
    #[doc = "0x4000 - Hart 0 time comparator register"]
    pub mtimecmp: MTIMECMP,
    #[doc = "0x4004 - Hart 0 time comparator register"]
    pub mtimecmph: MTIMECMPH,
    _reserved1: [u8; 32752usize],
    #[doc = "0xbff8 - Timer register"]
    pub mtime: MTIME,
    #[doc = "0xbffc - Timer register"]
    pub mtimeh: MTIMEH,
}
#[doc = "Hart 0 software interrupt register"]
pub struct MSIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hart 0 software interrupt register"]
pub mod msip;
#[doc = "Hart 0 time comparator register"]
pub struct MTIMECMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmp;
#[doc = "Hart 0 time comparator register"]
pub struct MTIMECMPH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmph;
#[doc = "Timer register"]
pub struct MTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer register"]
pub mod mtime;
#[doc = "Timer register"]
pub struct MTIMEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer register"]
pub mod mtimeh;
