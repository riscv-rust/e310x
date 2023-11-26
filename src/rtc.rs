#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    rtccfg: RTCCFG,
    _reserved1: [u8; 0x04],
    rtclo: RTCLO,
    rtchi: RTCHI,
    rtcs: RTCS,
    _reserved4: [u8; 0x0c],
    rtccmp: RTCCMP,
}
impl RegisterBlock {
    #[doc = "0x40 - RTC Configuration Register"]
    #[inline(always)]
    pub const fn rtccfg(&self) -> &RTCCFG {
        &self.rtccfg
    }
    #[doc = "0x48 - RTC Counter Low Register"]
    #[inline(always)]
    pub const fn rtclo(&self) -> &RTCLO {
        &self.rtclo
    }
    #[doc = "0x4c - RTC Counter High Register"]
    #[inline(always)]
    pub const fn rtchi(&self) -> &RTCHI {
        &self.rtchi
    }
    #[doc = "0x50 - RTC Scaled Counter Register"]
    #[inline(always)]
    pub const fn rtcs(&self) -> &RTCS {
        &self.rtcs
    }
    #[doc = "0x60 - RTC Compare Register"]
    #[inline(always)]
    pub const fn rtccmp(&self) -> &RTCCMP {
        &self.rtccmp
    }
}
#[doc = "rtccfg (rw) register accessor: RTC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccfg`]
module"]
pub type RTCCFG = crate::Reg<rtccfg::RTCCFG_SPEC>;
#[doc = "RTC Configuration Register"]
pub mod rtccfg;
#[doc = "rtclo (rw) register accessor: RTC Counter Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtclo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtclo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtclo`]
module"]
pub type RTCLO = crate::Reg<rtclo::RTCLO_SPEC>;
#[doc = "RTC Counter Low Register"]
pub mod rtclo;
#[doc = "rtchi (rw) register accessor: RTC Counter High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtchi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtchi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtchi`]
module"]
pub type RTCHI = crate::Reg<rtchi::RTCHI_SPEC>;
#[doc = "RTC Counter High Register"]
pub mod rtchi;
#[doc = "rtcs (rw) register accessor: RTC Scaled Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcs`]
module"]
pub type RTCS = crate::Reg<rtcs::RTCS_SPEC>;
#[doc = "RTC Scaled Counter Register"]
pub mod rtcs;
#[doc = "rtccmp (rw) register accessor: RTC Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccmp`]
module"]
pub type RTCCMP = crate::Reg<rtccmp::RTCCMP_SPEC>;
#[doc = "RTC Compare Register"]
pub mod rtccmp;
