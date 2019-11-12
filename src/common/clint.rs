#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hart 0 software interrupt register"]
    pub msip: MSIP,
    _reserved1: [u8; 16380usize],
    #[doc = "0x4000 - Hart 0 time comparator register"]
    pub mtimecmp: MTIMECMP,
    #[doc = "0x4004 - Hart 0 time comparator register"]
    pub mtimecmph: MTIMECMPH,
    _reserved3: [u8; 32752usize],
    #[doc = "0xbff8 - Timer register"]
    pub mtime: MTIME,
    #[doc = "0xbffc - Timer register"]
    pub mtimeh: MTIMEH,
}
#[doc = "Hart 0 software interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msip](msip) module"]
pub type MSIP = crate::Reg<u32, _MSIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSIP;
#[doc = "`read()` method returns [msip::R](msip::R) reader structure"]
impl crate::Readable for MSIP {}
#[doc = "`write(|w| ..)` method takes [msip::W](msip::W) writer structure"]
impl crate::Writable for MSIP {}
#[doc = "Hart 0 software interrupt register"]
pub mod msip;
#[doc = "Hart 0 time comparator register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtimecmp](mtimecmp) module"]
pub type MTIMECMP = crate::Reg<u32, _MTIMECMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIMECMP;
#[doc = "`read()` method returns [mtimecmp::R](mtimecmp::R) reader structure"]
impl crate::Readable for MTIMECMP {}
#[doc = "`write(|w| ..)` method takes [mtimecmp::W](mtimecmp::W) writer structure"]
impl crate::Writable for MTIMECMP {}
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmp;
#[doc = "Hart 0 time comparator register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtimecmph](mtimecmph) module"]
pub type MTIMECMPH = crate::Reg<u32, _MTIMECMPH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIMECMPH;
#[doc = "`read()` method returns [mtimecmph::R](mtimecmph::R) reader structure"]
impl crate::Readable for MTIMECMPH {}
#[doc = "`write(|w| ..)` method takes [mtimecmph::W](mtimecmph::W) writer structure"]
impl crate::Writable for MTIMECMPH {}
#[doc = "Hart 0 time comparator register"]
pub mod mtimecmph;
#[doc = "Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtime](mtime) module"]
pub type MTIME = crate::Reg<u32, _MTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIME;
#[doc = "`read()` method returns [mtime::R](mtime::R) reader structure"]
impl crate::Readable for MTIME {}
#[doc = "`write(|w| ..)` method takes [mtime::W](mtime::W) writer structure"]
impl crate::Writable for MTIME {}
#[doc = "Timer register"]
pub mod mtime;
#[doc = "Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtimeh](mtimeh) module"]
pub type MTIMEH = crate::Reg<u32, _MTIMEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIMEH;
#[doc = "`read()` method returns [mtimeh::R](mtimeh::R) reader structure"]
impl crate::Readable for MTIMEH {}
#[doc = "`write(|w| ..)` method takes [mtimeh::W](mtimeh::W) writer structure"]
impl crate::Writable for MTIMEH {}
#[doc = "Timer register"]
pub mod mtimeh;
