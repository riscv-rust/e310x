#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    lock: LOCK,
    clock: CLOCK,
    output_en: OUTPUT_EN,
    select: SELECT,
    write_en: WRITE_EN,
    mode: MODE,
    mrr: MRR,
    mpp: MPP,
    vrren: VRREN,
    vppen: VPPEN,
    addr: ADDR,
    data_in: DATA_IN,
    data_out: DATA_OUT,
    rsctrl: RSCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Programmed-I/O lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x04 - OTP device clock signal"]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x08 - OTP device output-enable signal"]
    #[inline(always)]
    pub const fn output_en(&self) -> &OUTPUT_EN {
        &self.output_en
    }
    #[doc = "0x0c - OTP device chip-select signal"]
    #[inline(always)]
    pub const fn select(&self) -> &SELECT {
        &self.select
    }
    #[doc = "0x10 - OTP device write-enable signal"]
    #[inline(always)]
    pub const fn write_en(&self) -> &WRITE_EN {
        &self.write_en
    }
    #[doc = "0x14 - OTP device mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x18 - OTP read-voltage regulator control"]
    #[inline(always)]
    pub const fn mrr(&self) -> &MRR {
        &self.mrr
    }
    #[doc = "0x1c - OTP write-voltage charge pump control"]
    #[inline(always)]
    pub const fn mpp(&self) -> &MPP {
        &self.mpp
    }
    #[doc = "0x20 - OTP read-voltage enable"]
    #[inline(always)]
    pub const fn vrren(&self) -> &VRREN {
        &self.vrren
    }
    #[doc = "0x24 - OTP write-voltage enable"]
    #[inline(always)]
    pub const fn vppen(&self) -> &VPPEN {
        &self.vppen
    }
    #[doc = "0x28 - OTP device address"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x2c - OTP device data input"]
    #[inline(always)]
    pub const fn data_in(&self) -> &DATA_IN {
        &self.data_in
    }
    #[doc = "0x30 - OTP device data output"]
    #[inline(always)]
    pub const fn data_out(&self) -> &DATA_OUT {
        &self.data_out
    }
    #[doc = "0x34 - OTP read sequencer control"]
    #[inline(always)]
    pub const fn rsctrl(&self) -> &RSCTRL {
        &self.rsctrl
    }
}
#[doc = "lock (rw) register accessor: Programmed-I/O lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Programmed-I/O lock register"]
pub mod lock;
#[doc = "clock (rw) register accessor: OTP device clock signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "OTP device clock signal"]
pub mod clock;
#[doc = "output_en (rw) register accessor: OTP device output-enable signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`output_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`output_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_en`]
module"]
pub type OUTPUT_EN = crate::Reg<output_en::OUTPUT_EN_SPEC>;
#[doc = "OTP device output-enable signal"]
pub mod output_en;
#[doc = "select (rw) register accessor: OTP device chip-select signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@select`]
module"]
pub type SELECT = crate::Reg<select::SELECT_SPEC>;
#[doc = "OTP device chip-select signal"]
pub mod select;
#[doc = "write_en (rw) register accessor: OTP device write-enable signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`write_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`write_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@write_en`]
module"]
pub type WRITE_EN = crate::Reg<write_en::WRITE_EN_SPEC>;
#[doc = "OTP device write-enable signal"]
pub mod write_en;
#[doc = "mode (rw) register accessor: OTP device mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "OTP device mode register"]
pub mod mode;
#[doc = "mrr (rw) register accessor: OTP read-voltage regulator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrr`]
module"]
pub type MRR = crate::Reg<mrr::MRR_SPEC>;
#[doc = "OTP read-voltage regulator control"]
pub mod mrr;
#[doc = "mpp (rw) register accessor: OTP write-voltage charge pump control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpp`]
module"]
pub type MPP = crate::Reg<mpp::MPP_SPEC>;
#[doc = "OTP write-voltage charge pump control"]
pub mod mpp;
#[doc = "vrren (rw) register accessor: OTP read-voltage enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrren`]
module"]
pub type VRREN = crate::Reg<vrren::VRREN_SPEC>;
#[doc = "OTP read-voltage enable"]
pub mod vrren;
#[doc = "vppen (rw) register accessor: OTP write-voltage enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vppen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vppen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vppen`]
module"]
pub type VPPEN = crate::Reg<vppen::VPPEN_SPEC>;
#[doc = "OTP write-voltage enable"]
pub mod vppen;
#[doc = "addr (rw) register accessor: OTP device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "OTP device address"]
pub mod addr;
#[doc = "data_in (rw) register accessor: OTP device data input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_in`]
module"]
pub type DATA_IN = crate::Reg<data_in::DATA_IN_SPEC>;
#[doc = "OTP device data input"]
pub mod data_in;
#[doc = "data_out (rw) register accessor: OTP device data output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_out`]
module"]
pub type DATA_OUT = crate::Reg<data_out::DATA_OUT_SPEC>;
#[doc = "OTP device data output"]
pub mod data_out;
#[doc = "rsctrl (rw) register accessor: OTP read sequencer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsctrl`]
module"]
pub type RSCTRL = crate::Reg<rsctrl::RSCTRL_SPEC>;
#[doc = "OTP read sequencer control"]
pub mod rsctrl;
