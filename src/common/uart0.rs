#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x04 - Receive Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x08 - Transmit Control Register"]
    pub txctrl: TXCTRL,
    #[doc = "0x0c - Receive Control Register"]
    pub rxctrl: RXCTRL,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x14 - Interrupt Pending Register"]
    pub ip: IP,
    #[doc = "0x18 - Baud Rate Divisor Register"]
    pub div: DIV,
}
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
#[doc = "Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txctrl](txctrl) module"]
pub type TXCTRL = crate::Reg<u32, _TXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCTRL;
#[doc = "`read()` method returns [txctrl::R](txctrl::R) reader structure"]
impl crate::Readable for TXCTRL {}
#[doc = "`write(|w| ..)` method takes [txctrl::W](txctrl::W) writer structure"]
impl crate::Writable for TXCTRL {}
#[doc = "Transmit Control Register"]
pub mod txctrl;
#[doc = "Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxctrl](rxctrl) module"]
pub type RXCTRL = crate::Reg<u32, _RXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCTRL;
#[doc = "`read()` method returns [rxctrl::R](rxctrl::R) reader structure"]
impl crate::Readable for RXCTRL {}
#[doc = "`write(|w| ..)` method takes [rxctrl::W](rxctrl::W) writer structure"]
impl crate::Writable for RXCTRL {}
#[doc = "Receive Control Register"]
pub mod rxctrl;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ip](ip) module"]
pub type IP = crate::Reg<u32, _IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IP;
#[doc = "`read()` method returns [ip::R](ip::R) reader structure"]
impl crate::Readable for IP {}
#[doc = "`write(|w| ..)` method takes [ip::W](ip::W) writer structure"]
impl crate::Writable for IP {}
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "Baud Rate Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [div](div) module"]
pub type DIV = crate::Reg<u32, _DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV;
#[doc = "`read()` method returns [div::R](div::R) reader structure"]
impl crate::Readable for DIV {}
#[doc = "`write(|w| ..)` method takes [div::W](div::W) writer structure"]
impl crate::Writable for DIV {}
#[doc = "Baud Rate Divisor Register"]
pub mod div;
