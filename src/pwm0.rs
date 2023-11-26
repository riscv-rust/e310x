#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cfg: CFG,
    _reserved1: [u8; 0x04],
    count: COUNT,
    _reserved2: [u8; 0x04],
    pwms: PWMS,
    _reserved3: [u8; 0x0c],
    cmp0: CMP0,
    cmp1: CMP1,
    cmp2: CMP2,
    cmp3: CMP3,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x08 - Counter Register"]
    #[inline(always)]
    pub const fn count(&self) -> &COUNT {
        &self.count
    }
    #[doc = "0x10 - Scaled Halfword Counter Register"]
    #[inline(always)]
    pub const fn pwms(&self) -> &PWMS {
        &self.pwms
    }
    #[doc = "0x20 - Compare Register"]
    #[inline(always)]
    pub const fn cmp0(&self) -> &CMP0 {
        &self.cmp0
    }
    #[doc = "0x24 - Compare Register"]
    #[inline(always)]
    pub const fn cmp1(&self) -> &CMP1 {
        &self.cmp1
    }
    #[doc = "0x28 - Compare Register"]
    #[inline(always)]
    pub const fn cmp2(&self) -> &CMP2 {
        &self.cmp2
    }
    #[doc = "0x2c - Compare Register"]
    #[inline(always)]
    pub const fn cmp3(&self) -> &CMP3 {
        &self.cmp3
    }
}
#[doc = "cfg (rw) register accessor: PWM Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "PWM Configuration Register"]
pub mod cfg;
#[doc = "count (rw) register accessor: Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Counter Register"]
pub mod count;
#[doc = "pwms (rw) register accessor: Scaled Halfword Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwms`]
module"]
pub type PWMS = crate::Reg<pwms::PWMS_SPEC>;
#[doc = "Scaled Halfword Counter Register"]
pub mod pwms;
#[doc = "cmp0 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0`]
module"]
pub type CMP0 = crate::Reg<cmp0::CMP0_SPEC>;
#[doc = "Compare Register"]
pub mod cmp0;
#[doc = "cmp1 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1`]
module"]
pub type CMP1 = crate::Reg<cmp1::CMP1_SPEC>;
#[doc = "Compare Register"]
pub mod cmp1;
#[doc = "cmp2 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2`]
module"]
pub type CMP2 = crate::Reg<cmp2::CMP2_SPEC>;
#[doc = "Compare Register"]
pub mod cmp2;
#[doc = "cmp3 (rw) register accessor: Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3`]
module"]
pub type CMP3 = crate::Reg<cmp3::CMP3_SPEC>;
#[doc = "Compare Register"]
pub mod cmp3;
