#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Serial Clock Divisor Register"]
    pub div: DIV,
    #[doc = "0x04 - Serial Clock Mode Register"]
    pub mode: MODE,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Chip Select ID Register"]
    pub csid: CSID,
    #[doc = "0x14 - Chip Select Default Register"]
    pub csdef: CSDEF,
    #[doc = "0x18 - Chip Select Mode Register"]
    pub csmode: CSMODE,
    _reserved5: [u8; 12usize],
    #[doc = "0x28 - Delay Control 0 Register"]
    pub delay0: DELAY0,
    #[doc = "0x2c - Delay Control 1 Register"]
    pub delay1: DELAY1,
    _reserved7: [u8; 16usize],
    #[doc = "0x40 - Frame Format Register"]
    pub fmt: FMT,
    _reserved8: [u8; 4usize],
    #[doc = "0x48 - Transmit Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x4c - Receive Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x50 - Transmit Watermark Register"]
    pub txmark: TXMARK,
    #[doc = "0x54 - Receive Watermark Register"]
    pub rxmark: RXMARK,
    _reserved12: [u8; 8usize],
    #[doc = "0x60 - SPI Flash Interface Control Register"]
    pub fctrl: FCTRL,
    #[doc = "0x64 - SPI Flash Instruction Format Register"]
    pub ffmt: FFMT,
    _reserved14: [u8; 8usize],
    #[doc = "0x70 - SPI Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x74 - SPI Interrupt Pending Register"]
    pub ip: IP,
}
#[doc = "Serial Clock Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [div](div) module"]
pub type DIV = crate::Reg<u32, _DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV;
#[doc = "`read()` method returns [div::R](div::R) reader structure"]
impl crate::Readable for DIV {}
#[doc = "`write(|w| ..)` method takes [div::W](div::W) writer structure"]
impl crate::Writable for DIV {}
#[doc = "Serial Clock Divisor Register"]
pub mod div;
#[doc = "Serial Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Serial Clock Mode Register"]
pub mod mode;
#[doc = "Chip Select ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csid](csid) module"]
pub type CSID = crate::Reg<u32, _CSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSID;
#[doc = "`read()` method returns [csid::R](csid::R) reader structure"]
impl crate::Readable for CSID {}
#[doc = "`write(|w| ..)` method takes [csid::W](csid::W) writer structure"]
impl crate::Writable for CSID {}
#[doc = "Chip Select ID Register"]
pub mod csid;
#[doc = "Chip Select Default Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csdef](csdef) module"]
pub type CSDEF = crate::Reg<u32, _CSDEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSDEF;
#[doc = "`read()` method returns [csdef::R](csdef::R) reader structure"]
impl crate::Readable for CSDEF {}
#[doc = "`write(|w| ..)` method takes [csdef::W](csdef::W) writer structure"]
impl crate::Writable for CSDEF {}
#[doc = "Chip Select Default Register"]
pub mod csdef;
#[doc = "Chip Select Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csmode](csmode) module"]
pub type CSMODE = crate::Reg<u32, _CSMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSMODE;
#[doc = "`read()` method returns [csmode::R](csmode::R) reader structure"]
impl crate::Readable for CSMODE {}
#[doc = "`write(|w| ..)` method takes [csmode::W](csmode::W) writer structure"]
impl crate::Writable for CSMODE {}
#[doc = "Chip Select Mode Register"]
pub mod csmode;
#[doc = "Delay Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [delay0](delay0) module"]
pub type DELAY0 = crate::Reg<u32, _DELAY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DELAY0;
#[doc = "`read()` method returns [delay0::R](delay0::R) reader structure"]
impl crate::Readable for DELAY0 {}
#[doc = "`write(|w| ..)` method takes [delay0::W](delay0::W) writer structure"]
impl crate::Writable for DELAY0 {}
#[doc = "Delay Control 0 Register"]
pub mod delay0;
#[doc = "Delay Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [delay1](delay1) module"]
pub type DELAY1 = crate::Reg<u32, _DELAY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DELAY1;
#[doc = "`read()` method returns [delay1::R](delay1::R) reader structure"]
impl crate::Readable for DELAY1 {}
#[doc = "`write(|w| ..)` method takes [delay1::W](delay1::W) writer structure"]
impl crate::Writable for DELAY1 {}
#[doc = "Delay Control 1 Register"]
pub mod delay1;
#[doc = "Frame Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmt](fmt) module"]
pub type FMT = crate::Reg<u32, _FMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMT;
#[doc = "`read()` method returns [fmt::R](fmt::R) reader structure"]
impl crate::Readable for FMT {}
#[doc = "`write(|w| ..)` method takes [fmt::W](fmt::W) writer structure"]
impl crate::Writable for FMT {}
#[doc = "Frame Format Register"]
pub mod fmt;
#[doc = "Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdata](txdata) module"]
pub type TXDATA = crate::Reg<u32, _TXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATA;
#[doc = "`read()` method returns [txdata::R](txdata::R) reader structure"]
impl crate::Readable for TXDATA {}
#[doc = "`write(|w| ..)` method takes [txdata::W](txdata::W) writer structure"]
impl crate::Writable for TXDATA {}
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdata](rxdata) module"]
pub type RXDATA = crate::Reg<u32, _RXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATA;
#[doc = "`read()` method returns [rxdata::R](rxdata::R) reader structure"]
impl crate::Readable for RXDATA {}
#[doc = "`write(|w| ..)` method takes [rxdata::W](rxdata::W) writer structure"]
impl crate::Writable for RXDATA {}
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "Transmit Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txmark](txmark) module"]
pub type TXMARK = crate::Reg<u32, _TXMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMARK;
#[doc = "`read()` method returns [txmark::R](txmark::R) reader structure"]
impl crate::Readable for TXMARK {}
#[doc = "`write(|w| ..)` method takes [txmark::W](txmark::W) writer structure"]
impl crate::Writable for TXMARK {}
#[doc = "Transmit Watermark Register"]
pub mod txmark;
#[doc = "Receive Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxmark](rxmark) module"]
pub type RXMARK = crate::Reg<u32, _RXMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMARK;
#[doc = "`read()` method returns [rxmark::R](rxmark::R) reader structure"]
impl crate::Readable for RXMARK {}
#[doc = "`write(|w| ..)` method takes [rxmark::W](rxmark::W) writer structure"]
impl crate::Writable for RXMARK {}
#[doc = "Receive Watermark Register"]
pub mod rxmark;
#[doc = "SPI Flash Interface Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctrl](fctrl) module"]
pub type FCTRL = crate::Reg<u32, _FCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTRL;
#[doc = "`read()` method returns [fctrl::R](fctrl::R) reader structure"]
impl crate::Readable for FCTRL {}
#[doc = "`write(|w| ..)` method takes [fctrl::W](fctrl::W) writer structure"]
impl crate::Writable for FCTRL {}
#[doc = "SPI Flash Interface Control Register"]
pub mod fctrl;
#[doc = "SPI Flash Instruction Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ffmt](ffmt) module"]
pub type FFMT = crate::Reg<u32, _FFMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFMT;
#[doc = "`read()` method returns [ffmt::R](ffmt::R) reader structure"]
impl crate::Readable for FFMT {}
#[doc = "`write(|w| ..)` method takes [ffmt::W](ffmt::W) writer structure"]
impl crate::Writable for FFMT {}
#[doc = "SPI Flash Instruction Format Register"]
pub mod ffmt;
#[doc = "SPI Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "SPI Interrupt Enable Register"]
pub mod ie;
#[doc = "SPI Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ip](ip) module"]
pub type IP = crate::Reg<u32, _IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IP;
#[doc = "`read()` method returns [ip::R](ip::R) reader structure"]
impl crate::Readable for IP {}
#[doc = "`write(|w| ..)` method takes [ip::W](ip::W) writer structure"]
impl crate::Writable for IP {}
#[doc = "SPI Interrupt Pending Register"]
pub mod ip;
