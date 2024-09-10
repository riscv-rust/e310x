#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msip: Msip,
    _reserved1: [u8; 0x3ffc],
    mtimecmp: Mtimecmp,
    mtimecmph: Mtimecmph,
    _reserved3: [u8; 0x7ff0],
    mtime: Mtime,
    mtimeh: Mtimeh,
}
impl RegisterBlock {
    #[doc = "0x00 - Hart 0 software interrupt register"]
    #[inline(always)]
    pub const fn msip(&self) -> &Msip {
        &self.msip
    }
    #[doc = "0x4000 - Hart 0 time comparator register"]
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &Mtimecmp {
        &self.mtimecmp
    }
    #[doc = "0x4004 - Hart 0 time comparator register"]
    #[inline(always)]
    pub const fn mtimecmph(&self) -> &Mtimecmph {
        &self.mtimecmph
    }
    #[doc = "0xbff8 - Timer register"]
    #[inline(always)]
    pub const fn mtime(&self) -> &Mtime {
        &self.mtime
    }
    #[doc = "0xbffc - Timer register"]
    #[inline(always)]
    pub const fn mtimeh(&self) -> &Mtimeh {
        &self.mtimeh
    }
}
#[doc = "msip (rw) register accessor: Hart 0 software interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
#[doc(alias = "msip")]
pub type Msip = crate::Reg<msip::MsipSpec>;
#[doc = "Hart 0 software interrupt register"]
pub mod msip;
#[doc = "mtimecmp (rw) register accessor: Hart 0 time comparator register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
#[doc(alias = "mtimecmp")]
pub type Mtimecmp = crate::Reg<mtimecmp::MtimecmpSpec>;
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmp;
#[doc = "mtimecmph (rw) register accessor: Hart 0 time comparator register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmph`]
module"]
#[doc(alias = "mtimecmph")]
pub type Mtimecmph = crate::Reg<mtimecmph::MtimecmphSpec>;
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmph;
#[doc = "mtime (rw) register accessor: Timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
#[doc(alias = "mtime")]
pub type Mtime = crate::Reg<mtime::MtimeSpec>;
#[doc = "Timer register"]
pub mod mtime;
#[doc = "mtimeh (rw) register accessor: Timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimeh`]
module"]
#[doc(alias = "mtimeh")]
pub type Mtimeh = crate::Reg<mtimeh::MtimehSpec>;
#[doc = "Timer register"]
pub mod mtimeh;
