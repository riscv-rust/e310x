#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration Register"]
    pub wdogcfg: WDOGCFG,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Watchdog Counter Register"]
    pub wdogcount: WDOGCOUNT,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Watchdog Scaled Counter Register"]
    pub wdogs: WDOGS,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - Watchdog Feed Register"]
    pub wdogfeed: WDOGFEED,
    #[doc = "0x1c - Watchdog Key Register"]
    pub wdogkey: WDOGKEY,
    #[doc = "0x20 - Watchdog Compare Register"]
    pub wdogcmp: WDOGCMP,
}
#[doc = "Watchdog Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogcfg](wdogcfg) module"]
pub type WDOGCFG = crate::Reg<u32, _WDOGCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGCFG;
#[doc = "`read()` method returns [wdogcfg::R](wdogcfg::R) reader structure"]
impl crate::Readable for WDOGCFG {}
#[doc = "`write(|w| ..)` method takes [wdogcfg::W](wdogcfg::W) writer structure"]
impl crate::Writable for WDOGCFG {}
#[doc = "Watchdog Configuration Register"]
pub mod wdogcfg;
#[doc = "Watchdog Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogcount](wdogcount) module"]
pub type WDOGCOUNT = crate::Reg<u32, _WDOGCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGCOUNT;
#[doc = "`read()` method returns [wdogcount::R](wdogcount::R) reader structure"]
impl crate::Readable for WDOGCOUNT {}
#[doc = "`write(|w| ..)` method takes [wdogcount::W](wdogcount::W) writer structure"]
impl crate::Writable for WDOGCOUNT {}
#[doc = "Watchdog Counter Register"]
pub mod wdogcount;
#[doc = "Watchdog Scaled Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogs](wdogs) module"]
pub type WDOGS = crate::Reg<u32, _WDOGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGS;
#[doc = "`read()` method returns [wdogs::R](wdogs::R) reader structure"]
impl crate::Readable for WDOGS {}
#[doc = "`write(|w| ..)` method takes [wdogs::W](wdogs::W) writer structure"]
impl crate::Writable for WDOGS {}
#[doc = "Watchdog Scaled Counter Register"]
pub mod wdogs;
#[doc = "Watchdog Feed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogfeed](wdogfeed) module"]
pub type WDOGFEED = crate::Reg<u32, _WDOGFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGFEED;
#[doc = "`read()` method returns [wdogfeed::R](wdogfeed::R) reader structure"]
impl crate::Readable for WDOGFEED {}
#[doc = "`write(|w| ..)` method takes [wdogfeed::W](wdogfeed::W) writer structure"]
impl crate::Writable for WDOGFEED {}
#[doc = "Watchdog Feed Register"]
pub mod wdogfeed;
#[doc = "Watchdog Key Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogkey](wdogkey) module"]
pub type WDOGKEY = crate::Reg<u32, _WDOGKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGKEY;
#[doc = "`write(|w| ..)` method takes [wdogkey::W](wdogkey::W) writer structure"]
impl crate::Writable for WDOGKEY {}
#[doc = "Watchdog Key Register"]
pub mod wdogkey;
#[doc = "Watchdog Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdogcmp](wdogcmp) module"]
pub type WDOGCMP = crate::Reg<u32, _WDOGCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOGCMP;
#[doc = "`read()` method returns [wdogcmp::R](wdogcmp::R) reader structure"]
impl crate::Readable for WDOGCMP {}
#[doc = "`write(|w| ..)` method takes [wdogcmp::W](wdogcmp::W) writer structure"]
impl crate::Writable for WDOGCMP {}
#[doc = "Watchdog Compare Register"]
pub mod wdogcmp;
