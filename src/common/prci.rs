#[doc = r"Register block"]
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
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrosccfg](hfrosccfg) module"]
pub type HFROSCCFG = crate::Reg<u32, _HFROSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFROSCCFG;
#[doc = "`read()` method returns [hfrosccfg::R](hfrosccfg::R) reader structure"]
impl crate::Readable for HFROSCCFG {}
#[doc = "`write(|w| ..)` method takes [hfrosccfg::W](hfrosccfg::W) writer structure"]
impl crate::Writable for HFROSCCFG {}
#[doc = "Clock Configuration Register"]
pub mod hfrosccfg;
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosccfg](hfxosccfg) module"]
pub type HFXOSCCFG = crate::Reg<u32, _HFXOSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOSCCFG;
#[doc = "`read()` method returns [hfxosccfg::R](hfxosccfg::R) reader structure"]
impl crate::Readable for HFXOSCCFG {}
#[doc = "`write(|w| ..)` method takes [hfxosccfg::W](hfxosccfg::W) writer structure"]
impl crate::Writable for HFXOSCCFG {}
#[doc = "Clock Configuration Register"]
pub mod hfxosccfg;
#[doc = "PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfg](pllcfg) module"]
pub type PLLCFG = crate::Reg<u32, _PLLCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFG;
#[doc = "`read()` method returns [pllcfg::R](pllcfg::R) reader structure"]
impl crate::Readable for PLLCFG {}
#[doc = "`write(|w| ..)` method takes [pllcfg::W](pllcfg::W) writer structure"]
impl crate::Writable for PLLCFG {}
#[doc = "PLL Configuration Register"]
pub mod pllcfg;
#[doc = "PLL Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plloutdiv](plloutdiv) module"]
pub type PLLOUTDIV = crate::Reg<u32, _PLLOUTDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLOUTDIV;
#[doc = "`read()` method returns [plloutdiv::R](plloutdiv::R) reader structure"]
impl crate::Readable for PLLOUTDIV {}
#[doc = "`write(|w| ..)` method takes [plloutdiv::W](plloutdiv::W) writer structure"]
impl crate::Writable for PLLOUTDIV {}
#[doc = "PLL Divider Register"]
pub mod plloutdiv;
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coreclkcfg](coreclkcfg) module"]
pub type CORECLKCFG = crate::Reg<u32, _CORECLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORECLKCFG;
#[doc = "`read()` method returns [coreclkcfg::R](coreclkcfg::R) reader structure"]
impl crate::Readable for CORECLKCFG {}
#[doc = "`write(|w| ..)` method takes [coreclkcfg::W](coreclkcfg::W) writer structure"]
impl crate::Writable for CORECLKCFG {}
#[doc = "Clock Configuration Register"]
pub mod coreclkcfg;
