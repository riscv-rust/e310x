#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    prer_lo: PRER_LO,
    prer_hi: PRER_HI,
    ctr: CTR,
    txr_rxr: TXR_RXR,
    _reserved_4_cr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Prescale register lo-byte"]
    #[inline(always)]
    pub const fn prer_lo(&self) -> &PRER_LO {
        &self.prer_lo
    }
    #[doc = "0x04 - Clock Prescale register hi-byte"]
    #[inline(always)]
    pub const fn prer_hi(&self) -> &PRER_HI {
        &self.prer_hi
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    #[doc = "0x0c - Transmit register / Receive register"]
    #[inline(always)]
    pub const fn txr_rxr(&self) -> &TXR_RXR {
        &self.txr_rxr
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Command register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Command register / Status register"]
    #[inline(always)]
    pub const fn cr_sr(&self) -> &CR_SR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
}
#[doc = "prer_lo (rw) register accessor: Clock Prescale register lo-byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer_lo`]
module"]
pub type PRER_LO = crate::Reg<prer_lo::PRER_LO_SPEC>;
#[doc = "Clock Prescale register lo-byte"]
pub mod prer_lo;
#[doc = "prer_hi (rw) register accessor: Clock Prescale register hi-byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer_hi`]
module"]
pub type PRER_HI = crate::Reg<prer_hi::PRER_HI_SPEC>;
#[doc = "Clock Prescale register hi-byte"]
pub mod prer_hi;
#[doc = "ctr (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Control register"]
pub mod ctr;
#[doc = "txr_rxr (rw) register accessor: Transmit register / Receive register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txr_rxr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txr_rxr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txr_rxr`]
module"]
pub type TXR_RXR = crate::Reg<txr_rxr::TXR_RXR_SPEC>;
#[doc = "Transmit register / Receive register"]
pub mod txr_rxr;
#[doc = "cr_sr (rw) register accessor: Command register / Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_sr`]
module"]
pub type CR_SR = crate::Reg<cr_sr::CR_SR_SPEC>;
#[doc = "Command register / Status register"]
pub mod cr_sr;
#[doc = "cr (w) register accessor: Command register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Command register"]
pub mod cr;
#[doc = "sr (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
