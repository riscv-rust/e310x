#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Serial Clock Divisor Register"]
    pub div: DIV,
    #[doc = "0x04 - Serial Clock Mode Register"]
    pub mode: MODE,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Chip Select ID Register"]
    pub csid: CSID,
    #[doc = "0x14 - Chip Select Default Register"]
    pub csdef: CSDEF,
    #[doc = "0x18 - Chip Select Mode Register"]
    pub csmode: CSMODE,
    _reserved1: [u8; 12usize],
    #[doc = "0x28 - Delay Control 0 Register"]
    pub delay0: DELAY0,
    #[doc = "0x2c - Delay Control 1 Register"]
    pub delay1: DELAY1,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - Frame Format Register"]
    pub fmt: FMT,
    _reserved3: [u8; 4usize],
    #[doc = "0x48 - Transmit Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x4c - Receive Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x50 - Transmit Watermark Register"]
    pub txmark: TXMARK,
    #[doc = "0x54 - Receive Watermark Register"]
    pub rxmark: RXMARK,
    _reserved4: [u8; 8usize],
    #[doc = "0x60 - SPI Flash Interface Control Register"]
    pub fctrl: FCTRL,
    #[doc = "0x64 - SPI Flash Instruction Format Register"]
    pub ffmt: FFMT,
    _reserved5: [u8; 8usize],
    #[doc = "0x70 - SPI Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x74 - SPI Interrupt Pending Register"]
    pub ip: IP,
}
#[doc = "Serial Clock Divisor Register"]
pub struct DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Clock Divisor Register"]
pub mod div;
#[doc = "Serial Clock Mode Register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Clock Mode Register"]
pub mod mode;
#[doc = "Chip Select ID Register"]
pub struct CSID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select ID Register"]
pub mod csid;
#[doc = "Chip Select Default Register"]
pub struct CSDEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Default Register"]
pub mod csdef;
#[doc = "Chip Select Mode Register"]
pub struct CSMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Mode Register"]
pub mod csmode;
#[doc = "Delay Control 0 Register"]
pub struct DELAY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Delay Control 0 Register"]
pub mod delay0;
#[doc = "Delay Control 1 Register"]
pub struct DELAY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Delay Control 1 Register"]
pub mod delay1;
#[doc = "Frame Format Register"]
pub struct FMT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Format Register"]
pub mod fmt;
#[doc = "Transmit Data Register"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "Receive Data Register"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "Transmit Watermark Register"]
pub struct TXMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Watermark Register"]
pub mod txmark;
#[doc = "Receive Watermark Register"]
pub struct RXMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Watermark Register"]
pub mod rxmark;
#[doc = "SPI Flash Interface Control Register"]
pub struct FCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Flash Interface Control Register"]
pub mod fctrl;
#[doc = "SPI Flash Instruction Format Register"]
pub struct FFMT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Flash Instruction Format Register"]
pub mod ffmt;
#[doc = "SPI Interrupt Enable Register"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Enable Register"]
pub mod ie;
#[doc = "SPI Interrupt Pending Register"]
pub struct IP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Pending Register"]
pub mod ip;
