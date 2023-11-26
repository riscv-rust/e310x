#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wdogcfg: WDOGCFG,
    _reserved1: [u8; 0x04],
    wdogcount: WDOGCOUNT,
    _reserved2: [u8; 0x04],
    wdogs: WDOGS,
    _reserved3: [u8; 0x04],
    wdogfeed: WDOGFEED,
    wdogkey: WDOGKEY,
    wdogcmp: WDOGCMP,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration Register"]
    #[inline(always)]
    pub const fn wdogcfg(&self) -> &WDOGCFG {
        &self.wdogcfg
    }
    #[doc = "0x08 - Watchdog Counter Register"]
    #[inline(always)]
    pub const fn wdogcount(&self) -> &WDOGCOUNT {
        &self.wdogcount
    }
    #[doc = "0x10 - Watchdog Scaled Counter Register"]
    #[inline(always)]
    pub const fn wdogs(&self) -> &WDOGS {
        &self.wdogs
    }
    #[doc = "0x18 - Watchdog Feed Register"]
    #[inline(always)]
    pub const fn wdogfeed(&self) -> &WDOGFEED {
        &self.wdogfeed
    }
    #[doc = "0x1c - Watchdog Key Register"]
    #[inline(always)]
    pub const fn wdogkey(&self) -> &WDOGKEY {
        &self.wdogkey
    }
    #[doc = "0x20 - Watchdog Compare Register"]
    #[inline(always)]
    pub const fn wdogcmp(&self) -> &WDOGCMP {
        &self.wdogcmp
    }
}
#[doc = "wdogcfg (rw) register accessor: Watchdog Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcfg`]
module"]
pub type WDOGCFG = crate::Reg<wdogcfg::WDOGCFG_SPEC>;
#[doc = "Watchdog Configuration Register"]
pub mod wdogcfg;
#[doc = "wdogcount (rw) register accessor: Watchdog Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcount::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcount::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcount`]
module"]
pub type WDOGCOUNT = crate::Reg<wdogcount::WDOGCOUNT_SPEC>;
#[doc = "Watchdog Counter Register"]
pub mod wdogcount;
#[doc = "wdogs (rw) register accessor: Watchdog Scaled Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogs`]
module"]
pub type WDOGS = crate::Reg<wdogs::WDOGS_SPEC>;
#[doc = "Watchdog Scaled Counter Register"]
pub mod wdogs;
#[doc = "wdogfeed (rw) register accessor: Watchdog Feed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogfeed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogfeed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogfeed`]
module"]
pub type WDOGFEED = crate::Reg<wdogfeed::WDOGFEED_SPEC>;
#[doc = "Watchdog Feed Register"]
pub mod wdogfeed;
#[doc = "wdogkey (w) register accessor: Watchdog Key Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogkey`]
module"]
pub type WDOGKEY = crate::Reg<wdogkey::WDOGKEY_SPEC>;
#[doc = "Watchdog Key Register"]
pub mod wdogkey;
#[doc = "wdogcmp (rw) register accessor: Watchdog Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcmp`]
module"]
pub type WDOGCMP = crate::Reg<wdogcmp::WDOGCMP_SPEC>;
#[doc = "Watchdog Compare Register"]
pub mod wdogcmp;
