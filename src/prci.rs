#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Configuration Register"]
    pub hfrosccfg: HFROSCCFG,
    #[doc = "0x04 - Clock Configuration Register"]
    pub hfxosccfg: HFXOSCCFG,
    #[doc = "0x08 - PLL Configuration Register"]
    pub pllcfg: PLLCFG,
    #[doc = "0x0c - PLL Divider Register"]
    pub plloutdiv: PLLOUTDIV,
    #[doc = "0x10 - Clock Configuration Register"]
    pub coreclkcfg: CORECLKCFG,
}
#[doc = "Clock Configuration Register"]
pub struct HFROSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration Register"]
pub mod hfrosccfg;
#[doc = "Clock Configuration Register"]
pub struct HFXOSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration Register"]
pub mod hfxosccfg;
#[doc = "PLL Configuration Register"]
pub struct PLLCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Configuration Register"]
pub mod pllcfg;
#[doc = "PLL Divider Register"]
pub struct PLLOUTDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Divider Register"]
pub mod plloutdiv;
#[doc = "Clock Configuration Register"]
pub struct CORECLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration Register"]
pub mod coreclkcfg;
