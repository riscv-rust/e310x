#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    msip: MSIP,
    _reserved1: [u8; 0x3ffc],
    mtimecmp: MTIMECMP,
    mtimecmph: MTIMECMPH,
    _reserved3: [u8; 0x7ff0],
    mtime: MTIME,
    mtimeh: MTIMEH,
}
impl RegisterBlock {
    #[doc = "0x00 - Hart 0 software interrupt register"]
    #[inline(always)]
    pub const fn msip(&self) -> &MSIP {
        &self.msip
    }
    #[doc = "0x4000 - Hart 0 time comparator register"]
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &MTIMECMP {
        &self.mtimecmp
    }
    #[doc = "0x4004 - Hart 0 time comparator register"]
    #[inline(always)]
    pub const fn mtimecmph(&self) -> &MTIMECMPH {
        &self.mtimecmph
    }
    #[doc = "0xbff8 - Timer register"]
    #[inline(always)]
    pub const fn mtime(&self) -> &MTIME {
        &self.mtime
    }
    #[doc = "0xbffc - Timer register"]
    #[inline(always)]
    pub const fn mtimeh(&self) -> &MTIMEH {
        &self.mtimeh
    }
}
#[doc = "msip (rw) register accessor: Hart 0 software interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Hart 0 software interrupt register"]
pub mod msip;
#[doc = "mtimecmp (rw) register accessor: Hart 0 time comparator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmp;
#[doc = "mtimecmph (rw) register accessor: Hart 0 time comparator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmph`]
module"]
pub type MTIMECMPH = crate::Reg<mtimecmph::MTIMECMPH_SPEC>;
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmph;
#[doc = "mtime (rw) register accessor: Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Timer register"]
pub mod mtime;
#[doc = "mtimeh (rw) register accessor: Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimeh`]
module"]
pub type MTIMEH = crate::Reg<mtimeh::MTIMEH_SPEC>;
#[doc = "Timer register"]
pub mod mtimeh;
