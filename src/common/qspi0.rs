#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Serial Clock Divisor Register"]
    pub sckdiv: crate::Reg<sckdiv::SCKDIV_SPEC>,
    #[doc = "0x04 - Serial Clock Mode Register"]
    pub sckmode: crate::Reg<sckmode::SCKMODE_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Chip Select ID Register"]
    pub csid: crate::Reg<csid::CSID_SPEC>,
    #[doc = "0x14 - Chip Select Default Register"]
    pub csdef: crate::Reg<csdef::CSDEF_SPEC>,
    #[doc = "0x18 - Chip Select Mode Register"]
    pub csmode: crate::Reg<csmode::CSMODE_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x28 - Delay Control 0 Register"]
    pub delay0: crate::Reg<delay0::DELAY0_SPEC>,
    #[doc = "0x2c - Delay Control 1 Register"]
    pub delay1: crate::Reg<delay1::DELAY1_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x40 - Frame Format Register"]
    pub fmt: crate::Reg<fmt::FMT_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x48 - Transmit Data Register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x4c - Receive Data Register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x50 - Transmit Watermark Register"]
    pub txmark: crate::Reg<txmark::TXMARK_SPEC>,
    #[doc = "0x54 - Receive Watermark Register"]
    pub rxmark: crate::Reg<rxmark::RXMARK_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x60 - SPI Flash Interface Control Register"]
    pub fctrl: crate::Reg<fctrl::FCTRL_SPEC>,
    #[doc = "0x64 - SPI Flash Instruction Format Register"]
    pub ffmt: crate::Reg<ffmt::FFMT_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x70 - SPI Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x74 - SPI Interrupt Pending Register"]
    pub ip: crate::Reg<ip::IP_SPEC>,
}
#[doc = "sckdiv register accessor: an alias for `Reg<SCKDIV_SPEC>`"]
pub type SCKDIV = crate::Reg<sckdiv::SCKDIV_SPEC>;
#[doc = "Serial Clock Divisor Register"]
pub mod sckdiv;
#[doc = "sckmode register accessor: an alias for `Reg<SCKMODE_SPEC>`"]
pub type SCKMODE = crate::Reg<sckmode::SCKMODE_SPEC>;
#[doc = "Serial Clock Mode Register"]
pub mod sckmode;
#[doc = "csid register accessor: an alias for `Reg<CSID_SPEC>`"]
pub type CSID = crate::Reg<csid::CSID_SPEC>;
#[doc = "Chip Select ID Register"]
pub mod csid;
#[doc = "csdef register accessor: an alias for `Reg<CSDEF_SPEC>`"]
pub type CSDEF = crate::Reg<csdef::CSDEF_SPEC>;
#[doc = "Chip Select Default Register"]
pub mod csdef;
#[doc = "csmode register accessor: an alias for `Reg<CSMODE_SPEC>`"]
pub type CSMODE = crate::Reg<csmode::CSMODE_SPEC>;
#[doc = "Chip Select Mode Register"]
pub mod csmode;
#[doc = "delay0 register accessor: an alias for `Reg<DELAY0_SPEC>`"]
pub type DELAY0 = crate::Reg<delay0::DELAY0_SPEC>;
#[doc = "Delay Control 0 Register"]
pub mod delay0;
#[doc = "delay1 register accessor: an alias for `Reg<DELAY1_SPEC>`"]
pub type DELAY1 = crate::Reg<delay1::DELAY1_SPEC>;
#[doc = "Delay Control 1 Register"]
pub mod delay1;
#[doc = "fmt register accessor: an alias for `Reg<FMT_SPEC>`"]
pub type FMT = crate::Reg<fmt::FMT_SPEC>;
#[doc = "Frame Format Register"]
pub mod fmt;
#[doc = "txdata register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "rxdata register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "txmark register accessor: an alias for `Reg<TXMARK_SPEC>`"]
pub type TXMARK = crate::Reg<txmark::TXMARK_SPEC>;
#[doc = "Transmit Watermark Register"]
pub mod txmark;
#[doc = "rxmark register accessor: an alias for `Reg<RXMARK_SPEC>`"]
pub type RXMARK = crate::Reg<rxmark::RXMARK_SPEC>;
#[doc = "Receive Watermark Register"]
pub mod rxmark;
#[doc = "fctrl register accessor: an alias for `Reg<FCTRL_SPEC>`"]
pub type FCTRL = crate::Reg<fctrl::FCTRL_SPEC>;
#[doc = "SPI Flash Interface Control Register"]
pub mod fctrl;
#[doc = "ffmt register accessor: an alias for `Reg<FFMT_SPEC>`"]
pub type FFMT = crate::Reg<ffmt::FFMT_SPEC>;
#[doc = "SPI Flash Instruction Format Register"]
pub mod ffmt;
#[doc = "ie register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "SPI Interrupt Enable Register"]
pub mod ie;
#[doc = "ip register accessor: an alias for `Reg<IP_SPEC>`"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "SPI Interrupt Pending Register"]
pub mod ip;
