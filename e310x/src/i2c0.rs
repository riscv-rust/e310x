#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    prer_lo: PrerLo,
    prer_hi: PrerHi,
    ctr: Ctr,
    txr_rxr: TxrRxr,
    _reserved_4_cr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Prescale register lo-byte"]
    #[inline(always)]
    pub const fn prer_lo(&self) -> &PrerLo {
        &self.prer_lo
    }
    #[doc = "0x04 - Clock Prescale register hi-byte"]
    #[inline(always)]
    pub const fn prer_hi(&self) -> &PrerHi {
        &self.prer_hi
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x0c - Transmit register / Receive register"]
    #[inline(always)]
    pub const fn txr_rxr(&self) -> &TxrRxr {
        &self.txr_rxr
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Command register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Command register / Status register"]
    #[inline(always)]
    pub const fn cr_sr(&self) -> &CrSr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
}
#[doc = "prer_lo (rw) register accessor: Clock Prescale register lo-byte\n\nYou can [`read`](crate::Reg::read) this register and get [`prer_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer_lo`]
module"]
#[doc(alias = "prer_lo")]
pub type PrerLo = crate::Reg<prer_lo::PrerLoSpec>;
#[doc = "Clock Prescale register lo-byte"]
pub mod prer_lo;
#[doc = "prer_hi (rw) register accessor: Clock Prescale register hi-byte\n\nYou can [`read`](crate::Reg::read) this register and get [`prer_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer_hi`]
module"]
#[doc(alias = "prer_hi")]
pub type PrerHi = crate::Reg<prer_hi::PrerHiSpec>;
#[doc = "Clock Prescale register hi-byte"]
pub mod prer_hi;
#[doc = "ctr (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "ctr")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Control register"]
pub mod ctr;
#[doc = "txr_rxr (rw) register accessor: Transmit register / Receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`txr_rxr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txr_rxr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txr_rxr`]
module"]
#[doc(alias = "txr_rxr")]
pub type TxrRxr = crate::Reg<txr_rxr::TxrRxrSpec>;
#[doc = "Transmit register / Receive register"]
pub mod txr_rxr;
#[doc = "cr_sr (rw) register accessor: Command register / Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_sr`]
module"]
#[doc(alias = "cr_sr")]
pub type CrSr = crate::Reg<cr_sr::CrSrSpec>;
#[doc = "Command register / Status register"]
pub mod cr_sr;
#[doc = "cr (w) register accessor: Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "cr")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Command register"]
pub mod cr;
#[doc = "sr (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "sr")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
