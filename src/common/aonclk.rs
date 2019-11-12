#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 112usize],
    #[doc = "0x70 - AON Clock Configuration Register"]
    pub lfrosccfg: LFROSCCFG,
}
#[doc = "AON Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfrosccfg](lfrosccfg) module"]
pub type LFROSCCFG = crate::Reg<u32, _LFROSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFROSCCFG;
#[doc = "`read()` method returns [lfrosccfg::R](lfrosccfg::R) reader structure"]
impl crate::Readable for LFROSCCFG {}
#[doc = "`write(|w| ..)` method takes [lfrosccfg::W](lfrosccfg::W) writer structure"]
impl crate::Writable for LFROSCCFG {}
#[doc = "AON Clock Configuration Register"]
pub mod lfrosccfg;
