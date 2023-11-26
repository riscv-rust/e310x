#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sckdiv: SCKDIV,
    sckmode: SCKMODE,
    _reserved2: [u8; 0x08],
    csid: CSID,
    csdef: CSDEF,
    csmode: CSMODE,
    _reserved5: [u8; 0x0c],
    delay0: DELAY0,
    delay1: DELAY1,
    _reserved7: [u8; 0x10],
    fmt: FMT,
    _reserved8: [u8; 0x04],
    txdata: TXDATA,
    rxdata: RXDATA,
    txmark: TXMARK,
    rxmark: RXMARK,
    _reserved12: [u8; 0x08],
    fctrl: FCTRL,
    ffmt: FFMT,
    _reserved14: [u8; 0x08],
    ie: IE,
    ip: IP,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial Clock Divisor Register"]
    #[inline(always)]
    pub const fn sckdiv(&self) -> &SCKDIV {
        &self.sckdiv
    }
    #[doc = "0x04 - Serial Clock Mode Register"]
    #[inline(always)]
    pub const fn sckmode(&self) -> &SCKMODE {
        &self.sckmode
    }
    #[doc = "0x10 - Chip Select ID Register"]
    #[inline(always)]
    pub const fn csid(&self) -> &CSID {
        &self.csid
    }
    #[doc = "0x14 - Chip Select Default Register"]
    #[inline(always)]
    pub const fn csdef(&self) -> &CSDEF {
        &self.csdef
    }
    #[doc = "0x18 - Chip Select Mode Register"]
    #[inline(always)]
    pub const fn csmode(&self) -> &CSMODE {
        &self.csmode
    }
    #[doc = "0x28 - Delay Control 0 Register"]
    #[inline(always)]
    pub const fn delay0(&self) -> &DELAY0 {
        &self.delay0
    }
    #[doc = "0x2c - Delay Control 1 Register"]
    #[inline(always)]
    pub const fn delay1(&self) -> &DELAY1 {
        &self.delay1
    }
    #[doc = "0x40 - Frame Format Register"]
    #[inline(always)]
    pub const fn fmt(&self) -> &FMT {
        &self.fmt
    }
    #[doc = "0x48 - Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &TXDATA {
        &self.txdata
    }
    #[doc = "0x4c - Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &RXDATA {
        &self.rxdata
    }
    #[doc = "0x50 - Transmit Watermark Register"]
    #[inline(always)]
    pub const fn txmark(&self) -> &TXMARK {
        &self.txmark
    }
    #[doc = "0x54 - Receive Watermark Register"]
    #[inline(always)]
    pub const fn rxmark(&self) -> &RXMARK {
        &self.rxmark
    }
    #[doc = "0x60 - SPI Flash Interface Control Register"]
    #[inline(always)]
    pub const fn fctrl(&self) -> &FCTRL {
        &self.fctrl
    }
    #[doc = "0x64 - SPI Flash Instruction Format Register"]
    #[inline(always)]
    pub const fn ffmt(&self) -> &FFMT {
        &self.ffmt
    }
    #[doc = "0x70 - SPI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x74 - SPI Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ip(&self) -> &IP {
        &self.ip
    }
}
#[doc = "sckdiv (rw) register accessor: Serial Clock Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckdiv`]
module"]
pub type SCKDIV = crate::Reg<sckdiv::SCKDIV_SPEC>;
#[doc = "Serial Clock Divisor Register"]
pub mod sckdiv;
#[doc = "sckmode (rw) register accessor: Serial Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sckmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sckmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckmode`]
module"]
pub type SCKMODE = crate::Reg<sckmode::SCKMODE_SPEC>;
#[doc = "Serial Clock Mode Register"]
pub mod sckmode;
#[doc = "csid (rw) register accessor: Chip Select ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csid`]
module"]
pub type CSID = crate::Reg<csid::CSID_SPEC>;
#[doc = "Chip Select ID Register"]
pub mod csid;
#[doc = "csdef (rw) register accessor: Chip Select Default Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csdef::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csdef::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csdef`]
module"]
pub type CSDEF = crate::Reg<csdef::CSDEF_SPEC>;
#[doc = "Chip Select Default Register"]
pub mod csdef;
#[doc = "csmode (rw) register accessor: Chip Select Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csmode`]
module"]
pub type CSMODE = crate::Reg<csmode::CSMODE_SPEC>;
#[doc = "Chip Select Mode Register"]
pub mod csmode;
#[doc = "delay0 (rw) register accessor: Delay Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay0`]
module"]
pub type DELAY0 = crate::Reg<delay0::DELAY0_SPEC>;
#[doc = "Delay Control 0 Register"]
pub mod delay0;
#[doc = "delay1 (rw) register accessor: Delay Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay1`]
module"]
pub type DELAY1 = crate::Reg<delay1::DELAY1_SPEC>;
#[doc = "Delay Control 1 Register"]
pub mod delay1;
#[doc = "fmt (rw) register accessor: Frame Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmt`]
module"]
pub type FMT = crate::Reg<fmt::FMT_SPEC>;
#[doc = "Frame Format Register"]
pub mod fmt;
#[doc = "txdata (rw) register accessor: Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "rxdata (rw) register accessor: Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "txmark (rw) register accessor: Transmit Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmark`]
module"]
pub type TXMARK = crate::Reg<txmark::TXMARK_SPEC>;
#[doc = "Transmit Watermark Register"]
pub mod txmark;
#[doc = "rxmark (rw) register accessor: Receive Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmark`]
module"]
pub type RXMARK = crate::Reg<rxmark::RXMARK_SPEC>;
#[doc = "Receive Watermark Register"]
pub mod rxmark;
#[doc = "fctrl (rw) register accessor: SPI Flash Interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl`]
module"]
pub type FCTRL = crate::Reg<fctrl::FCTRL_SPEC>;
#[doc = "SPI Flash Interface Control Register"]
pub mod fctrl;
#[doc = "ffmt (rw) register accessor: SPI Flash Instruction Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffmt`]
module"]
pub type FFMT = crate::Reg<ffmt::FFMT_SPEC>;
#[doc = "SPI Flash Instruction Format Register"]
pub mod ffmt;
#[doc = "ie (rw) register accessor: SPI Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "SPI Interrupt Enable Register"]
pub mod ie;
#[doc = "ip (rw) register accessor: SPI Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip`]
module"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "SPI Interrupt Pending Register"]
pub mod ip;
