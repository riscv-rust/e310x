#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    hfrosccfg: HFROSCCFG,
    hfxosccfg: HFXOSCCFG,
    pllcfg: PLLCFG,
    plloutdiv: PLLOUTDIV,
    coreclkcfg: CORECLKCFG,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn hfrosccfg(&self) -> &HFROSCCFG {
        &self.hfrosccfg
    }
    #[doc = "0x04 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn hfxosccfg(&self) -> &HFXOSCCFG {
        &self.hfxosccfg
    }
    #[doc = "0x08 - PLL Configuration Register"]
    #[inline(always)]
    pub const fn pllcfg(&self) -> &PLLCFG {
        &self.pllcfg
    }
    #[doc = "0x0c - PLL Divider Register"]
    #[inline(always)]
    pub const fn plloutdiv(&self) -> &PLLOUTDIV {
        &self.plloutdiv
    }
    #[doc = "0x10 - Clock Configuration Register"]
    #[inline(always)]
    pub const fn coreclkcfg(&self) -> &CORECLKCFG {
        &self.coreclkcfg
    }
}
#[doc = "hfrosccfg (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrosccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfrosccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrosccfg`]
module"]
pub type HFROSCCFG = crate::Reg<hfrosccfg::HFROSCCFG_SPEC>;
#[doc = "Clock Configuration Register"]
pub mod hfrosccfg;
#[doc = "hfxosccfg (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxosccfg`]
module"]
pub type HFXOSCCFG = crate::Reg<hfxosccfg::HFXOSCCFG_SPEC>;
#[doc = "Clock Configuration Register"]
pub mod hfxosccfg;
#[doc = "pllcfg (rw) register accessor: PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfg`]
module"]
pub type PLLCFG = crate::Reg<pllcfg::PLLCFG_SPEC>;
#[doc = "PLL Configuration Register"]
pub mod pllcfg;
#[doc = "plloutdiv (rw) register accessor: PLL Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plloutdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plloutdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plloutdiv`]
module"]
pub type PLLOUTDIV = crate::Reg<plloutdiv::PLLOUTDIV_SPEC>;
#[doc = "PLL Divider Register"]
pub mod plloutdiv;
#[doc = "coreclkcfg (rw) register accessor: Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coreclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coreclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coreclkcfg`]
module"]
pub type CORECLKCFG = crate::Reg<coreclkcfg::CORECLKCFG_SPEC>;
#[doc = "Clock Configuration Register"]
pub mod coreclkcfg;
