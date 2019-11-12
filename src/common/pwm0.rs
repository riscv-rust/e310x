#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - Counter Register"]
    pub count: COUNT,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Scaled Halfword Counter Register"]
    pub pwms: PWMS,
    _reserved3: [u8; 12usize],
    #[doc = "0x20 - Compare Register"]
    pub cmp0: CMP0,
    #[doc = "0x24 - Compare Register"]
    pub cmp1: CMP1,
    #[doc = "0x28 - Compare Register"]
    pub cmp2: CMP2,
    #[doc = "0x2c - Compare Register"]
    pub cmp3: CMP3,
}
#[doc = "PWM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "PWM Configuration Register"]
pub mod cfg;
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "`write(|w| ..)` method takes [count::W](count::W) writer structure"]
impl crate::Writable for COUNT {}
#[doc = "Counter Register"]
pub mod count;
#[doc = "Scaled Halfword Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwms](pwms) module"]
pub type PWMS = crate::Reg<u32, _PWMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMS;
#[doc = "`read()` method returns [pwms::R](pwms::R) reader structure"]
impl crate::Readable for PWMS {}
#[doc = "`write(|w| ..)` method takes [pwms::W](pwms::W) writer structure"]
impl crate::Writable for PWMS {}
#[doc = "Scaled Halfword Counter Register"]
pub mod pwms;
#[doc = "Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmp0](cmp0) module"]
pub type CMP0 = crate::Reg<u32, _CMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP0;
#[doc = "`read()` method returns [cmp0::R](cmp0::R) reader structure"]
impl crate::Readable for CMP0 {}
#[doc = "`write(|w| ..)` method takes [cmp0::W](cmp0::W) writer structure"]
impl crate::Writable for CMP0 {}
#[doc = "Compare Register"]
pub mod cmp0;
#[doc = "Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmp1](cmp1) module"]
pub type CMP1 = crate::Reg<u32, _CMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1;
#[doc = "`read()` method returns [cmp1::R](cmp1::R) reader structure"]
impl crate::Readable for CMP1 {}
#[doc = "`write(|w| ..)` method takes [cmp1::W](cmp1::W) writer structure"]
impl crate::Writable for CMP1 {}
#[doc = "Compare Register"]
pub mod cmp1;
#[doc = "Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmp2](cmp2) module"]
pub type CMP2 = crate::Reg<u32, _CMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2;
#[doc = "`read()` method returns [cmp2::R](cmp2::R) reader structure"]
impl crate::Readable for CMP2 {}
#[doc = "`write(|w| ..)` method takes [cmp2::W](cmp2::W) writer structure"]
impl crate::Writable for CMP2 {}
#[doc = "Compare Register"]
pub mod cmp2;
#[doc = "Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmp3](cmp3) module"]
pub type CMP3 = crate::Reg<u32, _CMP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3;
#[doc = "`read()` method returns [cmp3::R](cmp3::R) reader structure"]
impl crate::Readable for CMP3 {}
#[doc = "`write(|w| ..)` method takes [cmp3::W](cmp3::W) writer structure"]
impl crate::Writable for CMP3 {}
#[doc = "Compare Register"]
pub mod cmp3;
