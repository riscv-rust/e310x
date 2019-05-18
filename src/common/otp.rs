#[doc = r" Register block"]
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
#[doc = "Programmed-I/O lock register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmed-I/O lock register"]
pub mod lock;
#[doc = "OTP device clock signal"]
pub struct CLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device clock signal"]
pub mod clock;
#[doc = "OTP device output-enable signal"]
pub struct OUTPUT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device output-enable signal"]
pub mod output_en;
#[doc = "OTP device chip-select signal"]
pub struct SELECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device chip-select signal"]
pub mod select;
#[doc = "OTP device write-enable signal"]
pub struct WRITE_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device write-enable signal"]
pub mod write_en;
#[doc = "OTP device mode register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device mode register"]
pub mod mode;
#[doc = "OTP read-voltage regulator control"]
pub struct MRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP read-voltage regulator control"]
pub mod mrr;
#[doc = "OTP write-voltage charge pump control"]
pub struct MPP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP write-voltage charge pump control"]
pub mod mpp;
#[doc = "OTP read-voltage enable"]
pub struct VRREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP read-voltage enable"]
pub mod vrren;
#[doc = "OTP write-voltage enable"]
pub struct VPPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP write-voltage enable"]
pub mod vppen;
#[doc = "OTP device address"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device address"]
pub mod addr;
#[doc = "OTP device data input"]
pub struct DATA_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device data input"]
pub mod data_in;
#[doc = "OTP device data output"]
pub struct DATA_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP device data output"]
pub mod data_out;
#[doc = "OTP read sequencer control"]
pub struct RSCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP read sequencer control"]
pub mod rsctrl;
