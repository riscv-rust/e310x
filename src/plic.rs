#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    priority: [Priority; 53],
    _reserved1: [u8; 0x0f2c],
    pending: [Pending; 2],
    _reserved2: [u8; 0x0ff8],
    enable: [Enable; 2],
    _reserved3: [u8; 0x001f_dff8],
    threshold: Threshold,
    claim: Claim,
}
impl RegisterBlock {
    #[doc = "0x00..0xd4 - Interrupt Priority Register"]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &Priority {
        &self.priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xd4 - Interrupt Priority Register"]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &Priority> {
        self.priority.iter()
    }
    #[doc = "0x1000..0x1008 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &Pending {
        &self.pending[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1008 - Interrupt Pending Register"]
    #[inline(always)]
    pub fn pending_iter(&self) -> impl Iterator<Item = &Pending> {
        self.pending.iter()
    }
    #[doc = "0x2000..0x2008 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn enable(&self, n: usize) -> &Enable {
        &self.enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2008 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn enable_iter(&self) -> impl Iterator<Item = &Enable> {
        self.enable.iter()
    }
    #[doc = "0x200000 - Priority Threshold Register"]
    #[inline(always)]
    pub const fn threshold(&self) -> &Threshold {
        &self.threshold
    }
    #[doc = "0x200004 - Claim/Complete Register"]
    #[inline(always)]
    pub const fn claim(&self) -> &Claim {
        &self.claim
    }
}
#[doc = "priority (rw) register accessor: Interrupt Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`]
module"]
#[doc(alias = "priority")]
pub type Priority = crate::Reg<priority::PrioritySpec>;
#[doc = "Interrupt Priority Register"]
pub mod priority;
#[doc = "pending (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`]
module"]
#[doc(alias = "pending")]
pub type Pending = crate::Reg<pending::PendingSpec>;
#[doc = "Interrupt Pending Register"]
pub mod pending;
#[doc = "enable (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Interrupt Enable Register"]
pub mod enable;
#[doc = "threshold (rw) register accessor: Priority Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
#[doc(alias = "threshold")]
pub type Threshold = crate::Reg<threshold::ThresholdSpec>;
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "claim (rw) register accessor: Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim`]
module"]
#[doc(alias = "claim")]
pub type Claim = crate::Reg<claim::ClaimSpec>;
#[doc = "Claim/Complete Register"]
pub mod claim;
