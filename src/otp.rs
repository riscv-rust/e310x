#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Programmed-I/O lock register"]
    pub lock: LOCK,
    #[doc = "0x04 - OTP device clock signal"]
    pub clock: CLOCK,
    #[doc = "0x08 - OTP device output-enable signal"]
    pub output_en: OUTPUT_EN,
    #[doc = "0x0c - OTP device chip-select signal"]
    pub select: SELECT,
    #[doc = "0x10 - OTP device write-enable signal"]
    pub write_en: WRITE_EN,
    #[doc = "0x14 - OTP device mode register"]
    pub mode: MODE,
    #[doc = "0x18 - OTP read-voltage regulator control"]
    pub mrr: MRR,
    #[doc = "0x1c - OTP write-voltage charge pump control"]
    pub mpp: MPP,
    #[doc = "0x20 - OTP read-voltage enable"]
    pub vrren: VRREN,
    #[doc = "0x24 - OTP write-voltage enable"]
    pub vppen: VPPEN,
    #[doc = "0x28 - OTP device address"]
    pub addr: ADDR,
    #[doc = "0x2c - OTP device data input"]
    pub data_in: DATA_IN,
    #[doc = "0x30 - OTP device data output"]
    pub data_out: DATA_OUT,
    #[doc = "0x34 - OTP read sequencer control"]
    pub rsctrl: RSCTRL,
}
#[doc = "lock (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Programmed-I/O lock register"]
pub mod lock;
#[doc = "clock (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "OTP device clock signal"]
pub mod clock;
#[doc = "output_en (rw) register accessor: an alias for `Reg<OUTPUT_EN_SPEC>`"]
pub type OUTPUT_EN = crate::Reg<output_en::OUTPUT_EN_SPEC>;
#[doc = "OTP device output-enable signal"]
pub mod output_en;
#[doc = "select (rw) register accessor: an alias for `Reg<SELECT_SPEC>`"]
pub type SELECT = crate::Reg<select::SELECT_SPEC>;
#[doc = "OTP device chip-select signal"]
pub mod select;
#[doc = "write_en (rw) register accessor: an alias for `Reg<WRITE_EN_SPEC>`"]
pub type WRITE_EN = crate::Reg<write_en::WRITE_EN_SPEC>;
#[doc = "OTP device write-enable signal"]
pub mod write_en;
#[doc = "mode (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "OTP device mode register"]
pub mod mode;
#[doc = "mrr (rw) register accessor: an alias for `Reg<MRR_SPEC>`"]
pub type MRR = crate::Reg<mrr::MRR_SPEC>;
#[doc = "OTP read-voltage regulator control"]
pub mod mrr;
#[doc = "mpp (rw) register accessor: an alias for `Reg<MPP_SPEC>`"]
pub type MPP = crate::Reg<mpp::MPP_SPEC>;
#[doc = "OTP write-voltage charge pump control"]
pub mod mpp;
#[doc = "vrren (rw) register accessor: an alias for `Reg<VRREN_SPEC>`"]
pub type VRREN = crate::Reg<vrren::VRREN_SPEC>;
#[doc = "OTP read-voltage enable"]
pub mod vrren;
#[doc = "vppen (rw) register accessor: an alias for `Reg<VPPEN_SPEC>`"]
pub type VPPEN = crate::Reg<vppen::VPPEN_SPEC>;
#[doc = "OTP write-voltage enable"]
pub mod vppen;
#[doc = "addr (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "OTP device address"]
pub mod addr;
#[doc = "data_in (rw) register accessor: an alias for `Reg<DATA_IN_SPEC>`"]
pub type DATA_IN = crate::Reg<data_in::DATA_IN_SPEC>;
#[doc = "OTP device data input"]
pub mod data_in;
#[doc = "data_out (rw) register accessor: an alias for `Reg<DATA_OUT_SPEC>`"]
pub type DATA_OUT = crate::Reg<data_out::DATA_OUT_SPEC>;
#[doc = "OTP device data output"]
pub mod data_out;
#[doc = "rsctrl (rw) register accessor: an alias for `Reg<RSCTRL_SPEC>`"]
pub type RSCTRL = crate::Reg<rsctrl::RSCTRL_SPEC>;
#[doc = "OTP read sequencer control"]
pub mod rsctrl;
