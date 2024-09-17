#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hfrosccfg: Hfrosccfg,
    hfxosccfg: Hfxosccfg,
    pllcfg: Pllcfg,
    plloutdiv: Plloutdiv,
    coreclkcfg: Coreclkcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn hfrosccfg(&self) -> &Hfrosccfg {
        &self.hfrosccfg
    }
    #[doc = "0x04 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn hfxosccfg(&self) -> &Hfxosccfg {
        &self.hfxosccfg
    }
    #[doc = "0x08 - PLL Configuration Register"]
    #[inline(always)]
    pub const fn pllcfg(&self) -> &Pllcfg {
        &self.pllcfg
    }
    #[doc = "0x0c - PLL Divider Register"]
    #[inline(always)]
    pub const fn plloutdiv(&self) -> &Plloutdiv {
        &self.plloutdiv
    }
    #[doc = "0x10 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn coreclkcfg(&self) -> &Coreclkcfg {
        &self.coreclkcfg
    }
}
#[doc = "hfrosccfg (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrosccfg`]
module"]
#[doc(alias = "hfrosccfg")]
pub type Hfrosccfg = crate::Reg<hfrosccfg::HfrosccfgSpec>;
#[doc = "Clock Configuration Register"]
pub mod hfrosccfg;
#[doc = "hfxosccfg (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxosccfg`]
module"]
#[doc(alias = "hfxosccfg")]
pub type Hfxosccfg = crate::Reg<hfxosccfg::HfxosccfgSpec>;
#[doc = "Clock Configuration Register"]
pub mod hfxosccfg;
#[doc = "pllcfg (rw) register accessor: PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfg`]
module"]
#[doc(alias = "pllcfg")]
pub type Pllcfg = crate::Reg<pllcfg::PllcfgSpec>;
#[doc = "PLL Configuration Register"]
pub mod pllcfg;
#[doc = "plloutdiv (rw) register accessor: PLL Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plloutdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plloutdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plloutdiv`]
module"]
#[doc(alias = "plloutdiv")]
pub type Plloutdiv = crate::Reg<plloutdiv::PlloutdivSpec>;
#[doc = "PLL Divider Register"]
pub mod plloutdiv;
#[doc = "coreclkcfg (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`coreclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coreclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coreclkcfg`]
module"]
#[doc(alias = "coreclkcfg")]
pub type Coreclkcfg = crate::Reg<coreclkcfg::CoreclkcfgSpec>;
#[doc = "Clock Configuration Register"]
pub mod coreclkcfg;
