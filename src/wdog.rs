#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration Register"]
    pub wdogcfg: WDOGCFG,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Watchdog Counter Register"]
    pub wdogcount: WDOGCOUNT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Watchdog Scaled Counter Register"]
    pub wdogs: WDOGS,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Watchdog Feed Register"]
    pub wdogfeed: WDOGFEED,
    #[doc = "0x1c - Watchdog Key Register"]
    pub wdogkey: WDOGKEY,
    #[doc = "0x20 - Watchdog Compare Register"]
    pub wdogcmp: WDOGCMP,
}
#[doc = "wdogcfg (rw) register accessor: an alias for `Reg<WDOGCFG_SPEC>`"]
pub type WDOGCFG = crate::Reg<wdogcfg::WDOGCFG_SPEC>;
#[doc = "Watchdog Configuration Register"]
pub mod wdogcfg;
#[doc = "wdogcount (rw) register accessor: an alias for `Reg<WDOGCOUNT_SPEC>`"]
pub type WDOGCOUNT = crate::Reg<wdogcount::WDOGCOUNT_SPEC>;
#[doc = "Watchdog Counter Register"]
pub mod wdogcount;
#[doc = "wdogs (rw) register accessor: an alias for `Reg<WDOGS_SPEC>`"]
pub type WDOGS = crate::Reg<wdogs::WDOGS_SPEC>;
#[doc = "Watchdog Scaled Counter Register"]
pub mod wdogs;
#[doc = "wdogfeed (rw) register accessor: an alias for `Reg<WDOGFEED_SPEC>`"]
pub type WDOGFEED = crate::Reg<wdogfeed::WDOGFEED_SPEC>;
#[doc = "Watchdog Feed Register"]
pub mod wdogfeed;
#[doc = "wdogkey (w) register accessor: an alias for `Reg<WDOGKEY_SPEC>`"]
pub type WDOGKEY = crate::Reg<wdogkey::WDOGKEY_SPEC>;
#[doc = "Watchdog Key Register"]
pub mod wdogkey;
#[doc = "wdogcmp (rw) register accessor: an alias for `Reg<WDOGCMP_SPEC>`"]
pub type WDOGCMP = crate::Reg<wdogcmp::WDOGCMP_SPEC>;
#[doc = "Watchdog Compare Register"]
pub mod wdogcmp;
