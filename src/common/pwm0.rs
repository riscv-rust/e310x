#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - Counter Register"]
    pub count: COUNT,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Scaled Halfword Counter Register"]
    pub pwms: PWMS,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Compare Register"]
    pub cmp0: CMP0,
    #[doc = "0x24 - Compare Register"]
    pub cmp1: CMP1,
    #[doc = "0x28 - Compare Register"]
    pub cmp2: CMP2,
    #[doc = "0x2c - Compare Register"]
    pub cmp3: CMP3,
}
#[doc = "PWM Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Configuration Register"]
pub mod cfg;
#[doc = "Counter Register"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Register"]
pub mod count;
#[doc = "Scaled Halfword Counter Register"]
pub struct PWMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scaled Halfword Counter Register"]
pub mod pwms;
#[doc = "Compare Register"]
pub struct CMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register"]
pub mod cmp0;
#[doc = "Compare Register"]
pub struct CMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register"]
pub mod cmp1;
#[doc = "Compare Register"]
pub struct CMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register"]
pub mod cmp2;
#[doc = "Compare Register"]
pub struct CMP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register"]
pub mod cmp3;
