#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdogcfg: Wdogcfg,
    _reserved1: [u8; 0x04],
    wdogcount: Wdogcount,
    _reserved2: [u8; 0x04],
    wdogs: Wdogs,
    _reserved3: [u8; 0x04],
    wdogfeed: Wdogfeed,
    wdogkey: Wdogkey,
    wdogcmp: Wdogcmp,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration Register"]
    #[inline(always)]
    pub const fn wdogcfg(&self) -> &Wdogcfg {
        &self.wdogcfg
    }
    #[doc = "0x08 - Watchdog Counter Register"]
    #[inline(always)]
    pub const fn wdogcount(&self) -> &Wdogcount {
        &self.wdogcount
    }
    #[doc = "0x10 - Watchdog Scaled Counter Register"]
    #[inline(always)]
    pub const fn wdogs(&self) -> &Wdogs {
        &self.wdogs
    }
    #[doc = "0x18 - Watchdog Feed Register"]
    #[inline(always)]
    pub const fn wdogfeed(&self) -> &Wdogfeed {
        &self.wdogfeed
    }
    #[doc = "0x1c - Watchdog Key Register"]
    #[inline(always)]
    pub const fn wdogkey(&self) -> &Wdogkey {
        &self.wdogkey
    }
    #[doc = "0x20 - Watchdog Compare Register"]
    #[inline(always)]
    pub const fn wdogcmp(&self) -> &Wdogcmp {
        &self.wdogcmp
    }
}
#[doc = "wdogcfg (rw) register accessor: Watchdog Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcfg`]
module"]
#[doc(alias = "wdogcfg")]
pub type Wdogcfg = crate::Reg<wdogcfg::WdogcfgSpec>;
#[doc = "Watchdog Configuration Register"]
pub mod wdogcfg;
#[doc = "wdogcount (rw) register accessor: Watchdog Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcount`]
module"]
#[doc(alias = "wdogcount")]
pub type Wdogcount = crate::Reg<wdogcount::WdogcountSpec>;
#[doc = "Watchdog Counter Register"]
pub mod wdogcount;
#[doc = "wdogs (rw) register accessor: Watchdog Scaled Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogs`]
module"]
#[doc(alias = "wdogs")]
pub type Wdogs = crate::Reg<wdogs::WdogsSpec>;
#[doc = "Watchdog Scaled Counter Register"]
pub mod wdogs;
#[doc = "wdogfeed (rw) register accessor: Watchdog Feed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogfeed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogfeed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogfeed`]
module"]
#[doc(alias = "wdogfeed")]
pub type Wdogfeed = crate::Reg<wdogfeed::WdogfeedSpec>;
#[doc = "Watchdog Feed Register"]
pub mod wdogfeed;
#[doc = "wdogkey (w) register accessor: Watchdog Key Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogkey`]
module"]
#[doc(alias = "wdogkey")]
pub type Wdogkey = crate::Reg<wdogkey::WdogkeySpec>;
#[doc = "Watchdog Key Register"]
pub mod wdogkey;
#[doc = "wdogcmp (rw) register accessor: Watchdog Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcmp`]
module"]
#[doc(alias = "wdogcmp")]
pub type Wdogcmp = crate::Reg<wdogcmp::WdogcmpSpec>;
#[doc = "Watchdog Compare Register"]
pub mod wdogcmp;
