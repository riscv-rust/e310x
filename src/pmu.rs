#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    pmuwakepm: [PMUWAKEPM; 8],
    pmusleeppm: [PMUSLEEPPM; 8],
    pmuie: PMUIE,
    pmucause: PMUCAUSE,
    pmusleep: PMUSLEEP,
    pmukey: PMUKEY,
}
impl RegisterBlock {
    #[doc = "0x100..0x120 - PMU Wake Program Memory"]
    #[inline(always)]
    pub const fn pmuwakepm(&self, n: usize) -> &PMUWAKEPM {
        &self.pmuwakepm[n]
    }
    #[doc = "0x120..0x140 - PMU Sleep Program Memory"]
    #[inline(always)]
    pub const fn pmusleeppm(&self, n: usize) -> &PMUSLEEPPM {
        &self.pmusleeppm[n]
    }
    #[doc = "0x140 - PMU Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pmuie(&self) -> &PMUIE {
        &self.pmuie
    }
    #[doc = "0x144 - PMU Cause Register"]
    #[inline(always)]
    pub const fn pmucause(&self) -> &PMUCAUSE {
        &self.pmucause
    }
    #[doc = "0x148 - PMU Sleep Register"]
    #[inline(always)]
    pub const fn pmusleep(&self) -> &PMUSLEEP {
        &self.pmusleep
    }
    #[doc = "0x14c - PMU Key Register"]
    #[inline(always)]
    pub const fn pmukey(&self) -> &PMUKEY {
        &self.pmukey
    }
}
#[doc = "pmuwakepm (rw) register accessor: PMU Wake Program Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuwakepm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuwakepm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakepm`]
module"]
pub type PMUWAKEPM = crate::Reg<pmuwakepm::PMUWAKEPM_SPEC>;
#[doc = "PMU Wake Program Memory"]
pub mod pmuwakepm;
#[doc = "pmusleeppm (rw) register accessor: PMU Sleep Program Memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmusleeppm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleeppm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleeppm`]
module"]
pub type PMUSLEEPPM = crate::Reg<pmusleeppm::PMUSLEEPPM_SPEC>;
#[doc = "PMU Sleep Program Memory"]
pub mod pmusleeppm;
#[doc = "pmuie (rw) register accessor: PMU Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmuie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmuie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuie`]
module"]
pub type PMUIE = crate::Reg<pmuie::PMUIE_SPEC>;
#[doc = "PMU Interrupt Enable Register"]
pub mod pmuie;
#[doc = "pmucause (rw) register accessor: PMU Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucause`]
module"]
pub type PMUCAUSE = crate::Reg<pmucause::PMUCAUSE_SPEC>;
#[doc = "PMU Cause Register"]
pub mod pmucause;
#[doc = "pmusleep (w) register accessor: PMU Sleep Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmusleep::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleep`]
module"]
pub type PMUSLEEP = crate::Reg<pmusleep::PMUSLEEP_SPEC>;
#[doc = "PMU Sleep Register"]
pub mod pmusleep;
#[doc = "pmukey (rw) register accessor: PMU Key Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmukey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmukey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmukey`]
module"]
pub type PMUKEY = crate::Reg<pmukey::PMUKEY_SPEC>;
#[doc = "PMU Key Register"]
pub mod pmukey;
