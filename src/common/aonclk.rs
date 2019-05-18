#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 112usize],
    #[doc = "0x70 - AON Clock Configuration Register"]
    pub lfrosccfg: LFROSCCFG,
}
#[doc = "AON Clock Configuration Register"]
pub struct LFROSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AON Clock Configuration Register"]
pub mod lfrosccfg;
