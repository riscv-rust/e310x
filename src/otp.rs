#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lock: Lock,
    clock: Clock,
    output_en: OutputEn,
    select: Select,
    write_en: WriteEn,
    mode: Mode,
    mrr: Mrr,
    mpp: Mpp,
    vrren: Vrren,
    vppen: Vppen,
    addr: Addr,
    data_in: DataIn,
    data_out: DataOut,
    rsctrl: Rsctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Programmed-I/O lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x04 - OTP device clock signal"]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x08 - OTP device output-enable signal"]
    #[inline(always)]
    pub const fn output_en(&self) -> &OutputEn {
        &self.output_en
    }
    #[doc = "0x0c - OTP device chip-select signal"]
    #[inline(always)]
    pub const fn select(&self) -> &Select {
        &self.select
    }
    #[doc = "0x10 - OTP device write-enable signal"]
    #[inline(always)]
    pub const fn write_en(&self) -> &WriteEn {
        &self.write_en
    }
    #[doc = "0x14 - OTP device mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x18 - OTP read-voltage regulator control"]
    #[inline(always)]
    pub const fn mrr(&self) -> &Mrr {
        &self.mrr
    }
    #[doc = "0x1c - OTP write-voltage charge pump control"]
    #[inline(always)]
    pub const fn mpp(&self) -> &Mpp {
        &self.mpp
    }
    #[doc = "0x20 - OTP read-voltage enable"]
    #[inline(always)]
    pub const fn vrren(&self) -> &Vrren {
        &self.vrren
    }
    #[doc = "0x24 - OTP write-voltage enable"]
    #[inline(always)]
    pub const fn vppen(&self) -> &Vppen {
        &self.vppen
    }
    #[doc = "0x28 - OTP device address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x2c - OTP device data input"]
    #[inline(always)]
    pub const fn data_in(&self) -> &DataIn {
        &self.data_in
    }
    #[doc = "0x30 - OTP device data output"]
    #[inline(always)]
    pub const fn data_out(&self) -> &DataOut {
        &self.data_out
    }
    #[doc = "0x34 - OTP read sequencer control"]
    #[inline(always)]
    pub const fn rsctrl(&self) -> &Rsctrl {
        &self.rsctrl
    }
}
#[doc = "lock (rw) register accessor: Programmed-I/O lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "lock")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Programmed-I/O lock register"]
pub mod lock;
#[doc = "clock (rw) register accessor: OTP device clock signal\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
#[doc(alias = "clock")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "OTP device clock signal"]
pub mod clock;
#[doc = "output_en (rw) register accessor: OTP device output-enable signal\n\nYou can [`read`](crate::Reg::read) this register and get [`output_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output_en`]
module"]
#[doc(alias = "output_en")]
pub type OutputEn = crate::Reg<output_en::OutputEnSpec>;
#[doc = "OTP device output-enable signal"]
pub mod output_en;
#[doc = "select (rw) register accessor: OTP device chip-select signal\n\nYou can [`read`](crate::Reg::read) this register and get [`select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@select`]
module"]
#[doc(alias = "select")]
pub type Select = crate::Reg<select::SelectSpec>;
#[doc = "OTP device chip-select signal"]
pub mod select;
#[doc = "write_en (rw) register accessor: OTP device write-enable signal\n\nYou can [`read`](crate::Reg::read) this register and get [`write_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@write_en`]
module"]
#[doc(alias = "write_en")]
pub type WriteEn = crate::Reg<write_en::WriteEnSpec>;
#[doc = "OTP device write-enable signal"]
pub mod write_en;
#[doc = "mode (rw) register accessor: OTP device mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "mode")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "OTP device mode register"]
pub mod mode;
#[doc = "mrr (rw) register accessor: OTP read-voltage regulator control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrr`]
module"]
#[doc(alias = "mrr")]
pub type Mrr = crate::Reg<mrr::MrrSpec>;
#[doc = "OTP read-voltage regulator control"]
pub mod mrr;
#[doc = "mpp (rw) register accessor: OTP write-voltage charge pump control\n\nYou can [`read`](crate::Reg::read) this register and get [`mpp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpp`]
module"]
#[doc(alias = "mpp")]
pub type Mpp = crate::Reg<mpp::MppSpec>;
#[doc = "OTP write-voltage charge pump control"]
pub mod mpp;
#[doc = "vrren (rw) register accessor: OTP read-voltage enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vrren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrren`]
module"]
#[doc(alias = "vrren")]
pub type Vrren = crate::Reg<vrren::VrrenSpec>;
#[doc = "OTP read-voltage enable"]
pub mod vrren;
#[doc = "vppen (rw) register accessor: OTP write-voltage enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vppen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vppen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vppen`]
module"]
#[doc(alias = "vppen")]
pub type Vppen = crate::Reg<vppen::VppenSpec>;
#[doc = "OTP write-voltage enable"]
pub mod vppen;
#[doc = "addr (rw) register accessor: OTP device address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "addr")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "OTP device address"]
pub mod addr;
#[doc = "data_in (rw) register accessor: OTP device data input\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_in`]
module"]
#[doc(alias = "data_in")]
pub type DataIn = crate::Reg<data_in::DataInSpec>;
#[doc = "OTP device data input"]
pub mod data_in;
#[doc = "data_out (rw) register accessor: OTP device data output\n\nYou can [`read`](crate::Reg::read) this register and get [`data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_out`]
module"]
#[doc(alias = "data_out")]
pub type DataOut = crate::Reg<data_out::DataOutSpec>;
#[doc = "OTP device data output"]
pub mod data_out;
#[doc = "rsctrl (rw) register accessor: OTP read sequencer control\n\nYou can [`read`](crate::Reg::read) this register and get [`rsctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsctrl`]
module"]
#[doc(alias = "rsctrl")]
pub type Rsctrl = crate::Reg<rsctrl::RsctrlSpec>;
#[doc = "OTP read sequencer control"]
pub mod rsctrl;
