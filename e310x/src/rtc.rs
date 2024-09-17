#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    rtccfg: Rtccfg,
    _reserved1: [u8; 0x04],
    rtclo: Rtclo,
    rtchi: Rtchi,
    rtcs: Rtcs,
    _reserved4: [u8; 0x0c],
    rtccmp: Rtccmp,
}
impl RegisterBlock {
    #[doc = "0x40 - RTC Configuration Register"]
    #[inline(always)]
    pub const fn rtccfg(&self) -> &Rtccfg {
        &self.rtccfg
    }
    #[doc = "0x48 - RTC Counter Low Register"]
    #[inline(always)]
    pub const fn rtclo(&self) -> &Rtclo {
        &self.rtclo
    }
    #[doc = "0x4c - RTC Counter High Register"]
    #[inline(always)]
    pub const fn rtchi(&self) -> &Rtchi {
        &self.rtchi
    }
    #[doc = "0x50 - RTC Scaled Counter Register"]
    #[inline(always)]
    pub const fn rtcs(&self) -> &Rtcs {
        &self.rtcs
    }
    #[doc = "0x60 - RTC Compare Register"]
    #[inline(always)]
    pub const fn rtccmp(&self) -> &Rtccmp {
        &self.rtccmp
    }
}
#[doc = "rtccfg (rw) register accessor: RTC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccfg`]
module"]
#[doc(alias = "rtccfg")]
pub type Rtccfg = crate::Reg<rtccfg::RtccfgSpec>;
#[doc = "RTC Configuration Register"]
pub mod rtccfg;
#[doc = "rtclo (rw) register accessor: RTC Counter Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtclo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtclo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtclo`]
module"]
#[doc(alias = "rtclo")]
pub type Rtclo = crate::Reg<rtclo::RtcloSpec>;
#[doc = "RTC Counter Low Register"]
pub mod rtclo;
#[doc = "rtchi (rw) register accessor: RTC Counter High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtchi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtchi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtchi`]
module"]
#[doc(alias = "rtchi")]
pub type Rtchi = crate::Reg<rtchi::RtchiSpec>;
#[doc = "RTC Counter High Register"]
pub mod rtchi;
#[doc = "rtcs (rw) register accessor: RTC Scaled Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcs`]
module"]
#[doc(alias = "rtcs")]
pub type Rtcs = crate::Reg<rtcs::RtcsSpec>;
#[doc = "RTC Scaled Counter Register"]
pub mod rtcs;
#[doc = "rtccmp (rw) register accessor: RTC Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccmp`]
module"]
#[doc(alias = "rtccmp")]
pub type Rtccmp = crate::Reg<rtccmp::RtccmpSpec>;
#[doc = "RTC Compare Register"]
pub mod rtccmp;
