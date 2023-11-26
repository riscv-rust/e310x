#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    txdata: TXDATA,
    rxdata: RXDATA,
    txctrl: TXCTRL,
    rxctrl: RXCTRL,
    ie: IE,
    ip: IP,
    div: DIV,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &TXDATA {
        &self.txdata
    }
    #[doc = "0x04 - Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &RXDATA {
        &self.rxdata
    }
    #[doc = "0x08 - Transmit Control Register"]
    #[inline(always)]
    pub const fn txctrl(&self) -> &TXCTRL {
        &self.txctrl
    }
    #[doc = "0x0c - Receive Control Register"]
    #[inline(always)]
    pub const fn rxctrl(&self) -> &RXCTRL {
        &self.rxctrl
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x14 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn ip(&self) -> &IP {
        &self.ip
    }
    #[doc = "0x18 - Baud Rate Divisor Register"]
    #[inline(always)]
    pub const fn div(&self) -> &DIV {
        &self.div
    }
}
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
#[doc = "txctrl (rw) register accessor: Transmit Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl`]
module"]
pub type TXCTRL = crate::Reg<txctrl::TXCTRL_SPEC>;
#[doc = "Transmit Control Register"]
pub mod txctrl;
#[doc = "rxctrl (rw) register accessor: Receive Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl`]
module"]
pub type RXCTRL = crate::Reg<rxctrl::RXCTRL_SPEC>;
#[doc = "Receive Control Register"]
pub mod rxctrl;
#[doc = "ie (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "ip (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip`]
module"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "div (rw) register accessor: Baud Rate Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Baud Rate Divisor Register"]
pub mod div;
