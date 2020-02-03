#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Priority Register"]
    pub priority: [PRIORITY; 52],
    _reserved1: [u8; 3888usize],
    #[doc = "0x1000 - Interrupt Pending Register"]
    pub pending: [PENDING; 2],
    _reserved2: [u8; 4088usize],
    #[doc = "0x2000 - Interrupt Enable Register"]
    pub enable: [ENABLE; 2],
    _reserved3: [u8; 2088952usize],
    #[doc = "0x200000 - Priority Threshold Register"]
    pub threshold: THRESHOLD,
    #[doc = "0x200004 - Claim/Complete Register"]
    pub claim: CLAIM,
}
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priority](priority) module"]
pub type PRIORITY = crate::Reg<u32, _PRIORITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIORITY;
#[doc = "`read()` method returns [priority::R](priority::R) reader structure"]
impl crate::Readable for PRIORITY {}
#[doc = "`write(|w| ..)` method takes [priority::W](priority::W) writer structure"]
impl crate::Writable for PRIORITY {}
#[doc = "Interrupt Priority Register"]
pub mod priority;
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending](pending) module"]
pub type PENDING = crate::Reg<u32, _PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDING;
#[doc = "`read()` method returns [pending::R](pending::R) reader structure"]
impl crate::Readable for PENDING {}
#[doc = "`write(|w| ..)` method takes [pending::W](pending::W) writer structure"]
impl crate::Writable for PENDING {}
#[doc = "Interrupt Pending Register"]
pub mod pending;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Interrupt Enable Register"]
pub mod enable;
#[doc = "Priority Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold](threshold) module"]
pub type THRESHOLD = crate::Reg<u32, _THRESHOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESHOLD;
#[doc = "`read()` method returns [threshold::R](threshold::R) reader structure"]
impl crate::Readable for THRESHOLD {}
#[doc = "`write(|w| ..)` method takes [threshold::W](threshold::W) writer structure"]
impl crate::Writable for THRESHOLD {}
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "Claim/Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claim](claim) module"]
pub type CLAIM = crate::Reg<u32, _CLAIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIM;
#[doc = "`read()` method returns [claim::R](claim::R) reader structure"]
impl crate::Readable for CLAIM {}
#[doc = "`write(|w| ..)` method takes [claim::W](claim::W) writer structure"]
impl crate::Writable for CLAIM {}
#[doc = "Claim/Complete Register"]
pub mod claim;
