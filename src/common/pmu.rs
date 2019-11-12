#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - PMU Wake Program Memory"]
    pub pmuwakepm: [PMUWAKEPM; 8],
    #[doc = "0x120 - PMU Sleep Program Memory"]
    pub pmusleeppm: [PMUSLEEPPM; 8],
    #[doc = "0x140 - PMU Interrupt Enable Register"]
    pub pmuie: PMUIE,
    #[doc = "0x144 - PMU Cause Register"]
    pub pmucause: PMUCAUSE,
    #[doc = "0x148 - PMU Sleep Register"]
    pub pmusleep: PMUSLEEP,
    #[doc = "0x14c - PMU Key Register"]
    pub pmukey: PMUKEY,
}
#[doc = "PMU Wake Program Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmuwakepm](pmuwakepm) module"]
pub type PMUWAKEPM = crate::Reg<u32, _PMUWAKEPM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUWAKEPM;
#[doc = "`read()` method returns [pmuwakepm::R](pmuwakepm::R) reader structure"]
impl crate::Readable for PMUWAKEPM {}
#[doc = "`write(|w| ..)` method takes [pmuwakepm::W](pmuwakepm::W) writer structure"]
impl crate::Writable for PMUWAKEPM {}
#[doc = "PMU Wake Program Memory"]
pub mod pmuwakepm;
#[doc = "PMU Sleep Program Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmusleeppm](pmusleeppm) module"]
pub type PMUSLEEPPM = crate::Reg<u32, _PMUSLEEPPM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUSLEEPPM;
#[doc = "`read()` method returns [pmusleeppm::R](pmusleeppm::R) reader structure"]
impl crate::Readable for PMUSLEEPPM {}
#[doc = "`write(|w| ..)` method takes [pmusleeppm::W](pmusleeppm::W) writer structure"]
impl crate::Writable for PMUSLEEPPM {}
#[doc = "PMU Sleep Program Memory"]
pub mod pmusleeppm;
#[doc = "PMU Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmuie](pmuie) module"]
pub type PMUIE = crate::Reg<u32, _PMUIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUIE;
#[doc = "`read()` method returns [pmuie::R](pmuie::R) reader structure"]
impl crate::Readable for PMUIE {}
#[doc = "`write(|w| ..)` method takes [pmuie::W](pmuie::W) writer structure"]
impl crate::Writable for PMUIE {}
#[doc = "PMU Interrupt Enable Register"]
pub mod pmuie;
#[doc = "PMU Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmucause](pmucause) module"]
pub type PMUCAUSE = crate::Reg<u32, _PMUCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUCAUSE;
#[doc = "`read()` method returns [pmucause::R](pmucause::R) reader structure"]
impl crate::Readable for PMUCAUSE {}
#[doc = "`write(|w| ..)` method takes [pmucause::W](pmucause::W) writer structure"]
impl crate::Writable for PMUCAUSE {}
#[doc = "PMU Cause Register"]
pub mod pmucause;
#[doc = "PMU Sleep Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmusleep](pmusleep) module"]
pub type PMUSLEEP = crate::Reg<u32, _PMUSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUSLEEP;
#[doc = "`write(|w| ..)` method takes [pmusleep::W](pmusleep::W) writer structure"]
impl crate::Writable for PMUSLEEP {}
#[doc = "PMU Sleep Register"]
pub mod pmusleep;
#[doc = "PMU Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmukey](pmukey) module"]
pub type PMUKEY = crate::Reg<u32, _PMUKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUKEY;
#[doc = "`read()` method returns [pmukey::R](pmukey::R) reader structure"]
impl crate::Readable for PMUKEY {}
#[doc = "`write(|w| ..)` method takes [pmukey::W](pmukey::W) writer structure"]
impl crate::Writable for PMUKEY {}
#[doc = "PMU Key Register"]
pub mod pmukey;
