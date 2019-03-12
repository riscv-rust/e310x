#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration Register"]
    pub wdogcfg: WDOGCFG,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Watchdog Counter Register"]
    pub wdogcount: WDOGCOUNT,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - Watchdog Scaled Counter Register"]
    pub wdogs: WDOGS,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - Watchdog Feed Register"]
    pub wdogfeed: WDOGFEED,
    #[doc = "0x1c - Watchdog Key Register"]
    pub wdogkey: WDOGKEY,
    #[doc = "0x20 - Watchdog Compare Register"]
    pub wdogcmp: WDOGCMP,
}
#[doc = "Watchdog Configuration Register"]
pub struct WDOGCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Configuration Register"]
pub mod wdogcfg;
#[doc = "Watchdog Counter Register"]
pub struct WDOGCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Counter Register"]
pub mod wdogcount;
#[doc = "Watchdog Scaled Counter Register"]
pub struct WDOGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Scaled Counter Register"]
pub mod wdogs;
#[doc = "Watchdog Feed Register"]
pub struct WDOGFEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Feed Register"]
pub mod wdogfeed;
#[doc = "Watchdog Key Register"]
pub struct WDOGKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Key Register"]
pub mod wdogkey;
#[doc = "Watchdog Compare Register"]
pub struct WDOGCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Compare Register"]
pub mod wdogcmp;
