#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    pmuwakepm: [Pmuwakepm; 8],
    pmusleeppm: [Pmusleeppm; 8],
    pmuie: Pmuie,
    pmucause: Pmucause,
    pmusleep: Pmusleep,
    pmukey: Pmukey,
}
impl RegisterBlock {
    #[doc = "0x100..0x120 - PMU Wake Program Memory"]
    #[inline(always)]
    pub const fn pmuwakepm(&self, n: usize) -> &Pmuwakepm {
        &self.pmuwakepm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - PMU Wake Program Memory"]
    #[inline(always)]
    pub fn pmuwakepm_iter(&self) -> impl Iterator<Item = &Pmuwakepm> {
        self.pmuwakepm.iter()
    }
    #[doc = "0x120..0x140 - PMU Sleep Program Memory"]
    #[inline(always)]
    pub const fn pmusleeppm(&self, n: usize) -> &Pmusleeppm {
        &self.pmusleeppm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x140 - PMU Sleep Program Memory"]
    #[inline(always)]
    pub fn pmusleeppm_iter(&self) -> impl Iterator<Item = &Pmusleeppm> {
        self.pmusleeppm.iter()
    }
    #[doc = "0x140 - PMU Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pmuie(&self) -> &Pmuie {
        &self.pmuie
    }
    #[doc = "0x144 - PMU Cause Register"]
    #[inline(always)]
    pub const fn pmucause(&self) -> &Pmucause {
        &self.pmucause
    }
    #[doc = "0x148 - PMU Sleep Register"]
    #[inline(always)]
    pub const fn pmusleep(&self) -> &Pmusleep {
        &self.pmusleep
    }
    #[doc = "0x14c - PMU Key Register"]
    #[inline(always)]
    pub const fn pmukey(&self) -> &Pmukey {
        &self.pmukey
    }
}
#[doc = "pmuwakepm (rw) register accessor: PMU Wake Program Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuwakepm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuwakepm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuwakepm`]
module"]
#[doc(alias = "pmuwakepm")]
pub type Pmuwakepm = crate::Reg<pmuwakepm::PmuwakepmSpec>;
#[doc = "PMU Wake Program Memory"]
pub mod pmuwakepm;
#[doc = "pmusleeppm (rw) register accessor: PMU Sleep Program Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`pmusleeppm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmusleeppm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleeppm`]
module"]
#[doc(alias = "pmusleeppm")]
pub type Pmusleeppm = crate::Reg<pmusleeppm::PmusleeppmSpec>;
#[doc = "PMU Sleep Program Memory"]
pub mod pmusleeppm;
#[doc = "pmuie (rw) register accessor: PMU Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuie`]
module"]
#[doc(alias = "pmuie")]
pub type Pmuie = crate::Reg<pmuie::PmuieSpec>;
#[doc = "PMU Interrupt Enable Register"]
pub mod pmuie;
#[doc = "pmucause (rw) register accessor: PMU Cause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmucause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmucause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmucause`]
module"]
#[doc(alias = "pmucause")]
pub type Pmucause = crate::Reg<pmucause::PmucauseSpec>;
#[doc = "PMU Cause Register"]
pub mod pmucause;
#[doc = "pmusleep (w) register accessor: PMU Sleep Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmusleep::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmusleep`]
module"]
#[doc(alias = "pmusleep")]
pub type Pmusleep = crate::Reg<pmusleep::PmusleepSpec>;
#[doc = "PMU Sleep Register"]
pub mod pmusleep;
#[doc = "pmukey (rw) register accessor: PMU Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmukey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmukey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmukey`]
module"]
#[doc(alias = "pmukey")]
pub type Pmukey = crate::Reg<pmukey::PmukeySpec>;
#[doc = "PMU Key Register"]
pub mod pmukey;
