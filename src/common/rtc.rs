#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - RTC Configuration Register"]
    pub rtccfg: RTCCFG,
    _reserved1: [u8; 4usize],
    #[doc = "0x48 - RTC Counter Low Register"]
    pub rtclo: RTCLO,
    #[doc = "0x4c - RTC Counter High Register"]
    pub rtchi: RTCHI,
    #[doc = "0x50 - RTC Scaled Counter Register"]
    pub rtcs: RTCS,
    _reserved4: [u8; 12usize],
    #[doc = "0x60 - RTC Compare Register"]
    pub rtccmp: RTCCMP,
}
#[doc = "RTC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccfg](rtccfg) module"]
pub type RTCCFG = crate::Reg<u32, _RTCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCFG;
#[doc = "`read()` method returns [rtccfg::R](rtccfg::R) reader structure"]
impl crate::Readable for RTCCFG {}
#[doc = "`write(|w| ..)` method takes [rtccfg::W](rtccfg::W) writer structure"]
impl crate::Writable for RTCCFG {}
#[doc = "RTC Configuration Register"]
pub mod rtccfg;
#[doc = "RTC Counter Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtclo](rtclo) module"]
pub type RTCLO = crate::Reg<u32, _RTCLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCLO;
#[doc = "`read()` method returns [rtclo::R](rtclo::R) reader structure"]
impl crate::Readable for RTCLO {}
#[doc = "`write(|w| ..)` method takes [rtclo::W](rtclo::W) writer structure"]
impl crate::Writable for RTCLO {}
#[doc = "RTC Counter Low Register"]
pub mod rtclo;
#[doc = "RTC Counter High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtchi](rtchi) module"]
pub type RTCHI = crate::Reg<u32, _RTCHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCHI;
#[doc = "`read()` method returns [rtchi::R](rtchi::R) reader structure"]
impl crate::Readable for RTCHI {}
#[doc = "`write(|w| ..)` method takes [rtchi::W](rtchi::W) writer structure"]
impl crate::Writable for RTCHI {}
#[doc = "RTC Counter High Register"]
pub mod rtchi;
#[doc = "RTC Scaled Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcs](rtcs) module"]
pub type RTCS = crate::Reg<u32, _RTCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCS;
#[doc = "`read()` method returns [rtcs::R](rtcs::R) reader structure"]
impl crate::Readable for RTCS {}
#[doc = "`write(|w| ..)` method takes [rtcs::W](rtcs::W) writer structure"]
impl crate::Writable for RTCS {}
#[doc = "RTC Scaled Counter Register"]
pub mod rtcs;
#[doc = "RTC Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccmp](rtccmp) module"]
pub type RTCCMP = crate::Reg<u32, _RTCCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCMP;
#[doc = "`read()` method returns [rtccmp::R](rtccmp::R) reader structure"]
impl crate::Readable for RTCCMP {}
#[doc = "`write(|w| ..)` method takes [rtccmp::W](rtccmp::W) writer structure"]
impl crate::Writable for RTCCMP {}
#[doc = "RTC Compare Register"]
pub mod rtccmp;
