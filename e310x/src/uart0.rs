#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txdata: Txdata,
    rxdata: Rxdata,
    txctrl: Txctrl,
    rxctrl: Rxctrl,
    ie: Ie,
    ip: Ip,
    div: Div,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x04 - Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x08 - Transmit Control Register"]
    #[inline(always)]
    pub const fn txctrl(&self) -> &Txctrl {
        &self.txctrl
    }
    #[doc = "0x0c - Receive Control Register"]
    #[inline(always)]
    pub const fn rxctrl(&self) -> &Rxctrl {
        &self.rxctrl
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x14 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ip(&self) -> &Ip {
        &self.ip
    }
    #[doc = "0x18 - Baud Rate Divisor Register"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
}
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
#[doc = "txctrl (rw) register accessor: Transmit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl`]
module"]
#[doc(alias = "txctrl")]
pub type Txctrl = crate::Reg<txctrl::TxctrlSpec>;
#[doc = "Transmit Control Register"]
pub mod txctrl;
#[doc = "rxctrl (rw) register accessor: Receive Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl`]
module"]
#[doc(alias = "rxctrl")]
pub type Rxctrl = crate::Reg<rxctrl::RxctrlSpec>;
#[doc = "Receive Control Register"]
pub mod rxctrl;
#[doc = "ie (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "ie")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "ip (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip`]
module"]
#[doc(alias = "ip")]
pub type Ip = crate::Reg<ip::IpSpec>;
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "div (rw) register accessor: Baud Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "div")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Baud Rate Divisor Register"]
pub mod div;
