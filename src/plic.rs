#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    priority: [PRIORITY; 53],
    _reserved1: [u8; 0x0f2c],
    pending: [PENDING; 2],
    _reserved2: [u8; 0x0ff8],
    enable: [ENABLE; 2],
    _reserved3: [u8; 0x001f_dff8],
    threshold: THRESHOLD,
    claim: CLAIM,
}
impl RegisterBlock {
    #[doc = "0x00..0xd4 - Interrupt Priority Register"]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &PRIORITY {
        &self.priority[n]
    }
    #[doc = "0x1000..0x1008 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &PENDING {
        &self.pending[n]
    }
    #[doc = "0x2000..0x2008 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn enable(&self, n: usize) -> &ENABLE {
        &self.enable[n]
    }
    #[doc = "0x200000 - Priority Threshold Register"]
    #[inline(always)]
    pub const fn threshold(&self) -> &THRESHOLD {
        &self.threshold
    }
    #[doc = "0x200004 - Claim/Complete Register"]
    #[inline(always)]
    pub const fn claim(&self) -> &CLAIM {
        &self.claim
    }
}
#[doc = "priority (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
pub type PRIORITY = crate::Reg<priority::PRIORITY_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod priority;
#[doc = "pending (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`]
module"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod pending;
#[doc = "enable (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod enable;
#[doc = "threshold (rw) register accessor: Priority Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "claim (rw) register accessor: Claim/Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim`]
module"]
pub type CLAIM = crate::Reg<claim::CLAIM_SPEC>;
#[doc = "Claim/Complete Register"]
pub mod claim;
