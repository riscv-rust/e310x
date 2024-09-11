#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sckdiv: Sckdiv,
    sckmode: Sckmode,
    _reserved2: [u8; 0x08],
    csid: Csid,
    csdef: Csdef,
    csmode: Csmode,
    _reserved5: [u8; 0x0c],
    delay0: Delay0,
    delay1: Delay1,
    _reserved7: [u8; 0x10],
    fmt: Fmt,
    _reserved8: [u8; 0x04],
    txdata: Txdata,
    rxdata: Rxdata,
    txmark: Txmark,
    rxmark: Rxmark,
    _reserved12: [u8; 0x08],
    fctrl: Fctrl,
    ffmt: Ffmt,
    _reserved14: [u8; 0x08],
    ie: Ie,
    ip: Ip,
}
impl RegisterBlock {
    #[doc = "0x00 - Serial Clock Divisor Register"]
    #[inline(always)]
    pub const fn sckdiv(&self) -> &Sckdiv {
        &self.sckdiv
    }
    #[doc = "0x04 - Serial Clock Mode Register"]
    #[inline(always)]
    pub const fn sckmode(&self) -> &Sckmode {
        &self.sckmode
    }
    #[doc = "0x10 - Chip Select ID Register"]
    #[inline(always)]
    pub const fn csid(&self) -> &Csid {
        &self.csid
    }
    #[doc = "0x14 - Chip Select Default Register"]
    #[inline(always)]
    pub const fn csdef(&self) -> &Csdef {
        &self.csdef
    }
    #[doc = "0x18 - Chip Select Mode Register"]
    #[inline(always)]
    pub const fn csmode(&self) -> &Csmode {
        &self.csmode
    }
    #[doc = "0x28 - Delay Control 0 Register"]
    #[inline(always)]
    pub const fn delay0(&self) -> &Delay0 {
        &self.delay0
    }
    #[doc = "0x2c - Delay Control 1 Register"]
    #[inline(always)]
    pub const fn delay1(&self) -> &Delay1 {
        &self.delay1
    }
    #[doc = "0x40 - Frame Format Register"]
    #[inline(always)]
    pub const fn fmt(&self) -> &Fmt {
        &self.fmt
    }
    #[doc = "0x48 - Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x4c - Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x50 - Transmit Watermark Register"]
    #[inline(always)]
    pub const fn txmark(&self) -> &Txmark {
        &self.txmark
    }
    #[doc = "0x54 - Receive Watermark Register"]
    #[inline(always)]
    pub const fn rxmark(&self) -> &Rxmark {
        &self.rxmark
    }
    #[doc = "0x60 - SPI Flash Interface Control Register"]
    #[inline(always)]
    pub const fn fctrl(&self) -> &Fctrl {
        &self.fctrl
    }
    #[doc = "0x64 - SPI Flash Instruction Format Register"]
    #[inline(always)]
    pub const fn ffmt(&self) -> &Ffmt {
        &self.ffmt
    }
    #[doc = "0x70 - SPI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x74 - SPI Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ip(&self) -> &Ip {
        &self.ip
    }
}
#[doc = "sckdiv (rw) register accessor: Serial Clock Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckdiv`]
module"]
#[doc(alias = "sckdiv")]
pub type Sckdiv = crate::Reg<sckdiv::SckdivSpec>;
#[doc = "Serial Clock Divisor Register"]
pub mod sckdiv;
#[doc = "sckmode (rw) register accessor: Serial Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sckmode`]
module"]
#[doc(alias = "sckmode")]
pub type Sckmode = crate::Reg<sckmode::SckmodeSpec>;
#[doc = "Serial Clock Mode Register"]
pub mod sckmode;
#[doc = "csid (rw) register accessor: Chip Select ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csid`]
module"]
#[doc(alias = "csid")]
pub type Csid = crate::Reg<csid::CsidSpec>;
#[doc = "Chip Select ID Register"]
pub mod csid;
#[doc = "csdef (rw) register accessor: Chip Select Default Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csdef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csdef`]
module"]
#[doc(alias = "csdef")]
pub type Csdef = crate::Reg<csdef::CsdefSpec>;
#[doc = "Chip Select Default Register"]
pub mod csdef;
#[doc = "csmode (rw) register accessor: Chip Select Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csmode`]
module"]
#[doc(alias = "csmode")]
pub type Csmode = crate::Reg<csmode::CsmodeSpec>;
#[doc = "Chip Select Mode Register"]
pub mod csmode;
#[doc = "delay0 (rw) register accessor: Delay Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`delay0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay0`]
module"]
#[doc(alias = "delay0")]
pub type Delay0 = crate::Reg<delay0::Delay0Spec>;
#[doc = "Delay Control 0 Register"]
pub mod delay0;
#[doc = "delay1 (rw) register accessor: Delay Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`delay1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay1`]
module"]
#[doc(alias = "delay1")]
pub type Delay1 = crate::Reg<delay1::Delay1Spec>;
#[doc = "Delay Control 1 Register"]
pub mod delay1;
#[doc = "fmt (rw) register accessor: Frame Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmt`]
module"]
#[doc(alias = "fmt")]
pub type Fmt = crate::Reg<fmt::FmtSpec>;
#[doc = "Frame Format Register"]
pub mod fmt;
#[doc = "txdata (rw) register accessor: Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "txdata")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "rxdata (rw) register accessor: Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "rxdata")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "txmark (rw) register accessor: Transmit Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txmark::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txmark::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmark`]
module"]
#[doc(alias = "txmark")]
pub type Txmark = crate::Reg<txmark::TxmarkSpec>;
#[doc = "Transmit Watermark Register"]
pub mod txmark;
#[doc = "rxmark (rw) register accessor: Receive Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmark::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmark::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmark`]
module"]
#[doc(alias = "rxmark")]
pub type Rxmark = crate::Reg<rxmark::RxmarkSpec>;
#[doc = "Receive Watermark Register"]
pub mod rxmark;
#[doc = "fctrl (rw) register accessor: SPI Flash Interface Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl`]
module"]
#[doc(alias = "fctrl")]
pub type Fctrl = crate::Reg<fctrl::FctrlSpec>;
#[doc = "SPI Flash Interface Control Register"]
pub mod fctrl;
#[doc = "ffmt (rw) register accessor: SPI Flash Instruction Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffmt`]
module"]
#[doc(alias = "ffmt")]
pub type Ffmt = crate::Reg<ffmt::FfmtSpec>;
#[doc = "SPI Flash Instruction Format Register"]
pub mod ffmt;
#[doc = "ie (rw) register accessor: SPI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "ie")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "SPI Interrupt Enable Register"]
pub mod ie;
#[doc = "ip (rw) register accessor: SPI Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip`]
module"]
#[doc(alias = "ip")]
pub type Ip = crate::Reg<ip::IpSpec>;
#[doc = "SPI Interrupt Pending Register"]
pub mod ip;
