#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    _reserved1: [u8; 0x04],
    count: Count,
    _reserved2: [u8; 0x04],
    pwms: Pwms,
    _reserved3: [u8; 0x0c],
    cmp0: Cmp0,
    cmp1: Cmp1,
    cmp2: Cmp2,
    cmp3: Cmp3,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Counter Register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x10 - Scaled Halfword Counter Register"]
    #[inline(always)]
    pub const fn pwms(&self) -> &Pwms {
        &self.pwms
    }
    #[doc = "0x20 - Compare Register"]
    #[inline(always)]
    pub const fn cmp0(&self) -> &Cmp0 {
        &self.cmp0
    }
    #[doc = "0x24 - Compare Register"]
    #[inline(always)]
    pub const fn cmp1(&self) -> &Cmp1 {
        &self.cmp1
    }
    #[doc = "0x28 - Compare Register"]
    #[inline(always)]
    pub const fn cmp2(&self) -> &Cmp2 {
        &self.cmp2
    }
    #[doc = "0x2c - Compare Register"]
    #[inline(always)]
    pub const fn cmp3(&self) -> &Cmp3 {
        &self.cmp3
    }
}
#[doc = "cfg (rw) register accessor: PWM Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "PWM Configuration Register"]
pub mod cfg;
#[doc = "count (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "count")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "Counter Register"]
pub mod count;
#[doc = "pwms (rw) register accessor: Scaled Halfword Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwms`]
module"]
#[doc(alias = "pwms")]
pub type Pwms = crate::Reg<pwms::PwmsSpec>;
#[doc = "Scaled Halfword Counter Register"]
pub mod pwms;
#[doc = "cmp0 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0`]
module"]
#[doc(alias = "cmp0")]
pub type Cmp0 = crate::Reg<cmp0::Cmp0Spec>;
#[doc = "Compare Register"]
pub mod cmp0;
#[doc = "cmp1 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1`]
module"]
#[doc(alias = "cmp1")]
pub type Cmp1 = crate::Reg<cmp1::Cmp1Spec>;
#[doc = "Compare Register"]
pub mod cmp1;
#[doc = "cmp2 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2`]
module"]
#[doc(alias = "cmp2")]
pub type Cmp2 = crate::Reg<cmp2::Cmp2Spec>;
#[doc = "Compare Register"]
pub mod cmp2;
#[doc = "cmp3 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3`]
module"]
#[doc(alias = "cmp3")]
pub type Cmp3 = crate::Reg<cmp3::Cmp3Spec>;
#[doc = "Compare Register"]
pub mod cmp3;
