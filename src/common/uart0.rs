#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit Data Register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x04 - Receive Data Register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x08 - Transmit Control Register"]
    pub txctrl: crate::Reg<txctrl::TXCTRL_SPEC>,
    #[doc = "0x0c - Receive Control Register"]
    pub rxctrl: crate::Reg<rxctrl::RXCTRL_SPEC>,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x14 - Interrupt Pending Register"]
    pub ip: crate::Reg<ip::IP_SPEC>,
    #[doc = "0x18 - Baud Rate Divisor Register"]
    pub div: crate::Reg<div::DIV_SPEC>,
}
#[doc = "txdata register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "rxdata register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "txctrl register accessor: an alias for `Reg<TXCTRL_SPEC>`"]
pub type TXCTRL = crate::Reg<txctrl::TXCTRL_SPEC>;
#[doc = "Transmit Control Register"]
pub mod txctrl;
#[doc = "rxctrl register accessor: an alias for `Reg<RXCTRL_SPEC>`"]
pub type RXCTRL = crate::Reg<rxctrl::RXCTRL_SPEC>;
#[doc = "Receive Control Register"]
pub mod rxctrl;
#[doc = "ie register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "ip register accessor: an alias for `Reg<IP_SPEC>`"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "div register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Baud Rate Divisor Register"]
pub mod div;
