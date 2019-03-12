#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - PMU Wake Program Memory"]
    pub pmuwakepm: [PMUWAKEPM; 8],
    #[doc = "0x120 - PMU Sleep Program Memory"]
    pub pmusleeppm: [PMUSLEEPPM; 8],
    #[doc = "0x140 - PMU Interrupt Enable Register"]
    pub pmuie: PMUIE,
    #[doc = "0x144 - PMU Cause Register"]
    pub pmucause: PMUCAUSE,
    #[doc = "0x148 - PMU Sleep Register"]
    pub pmusleep: PMUSLEEP,
    #[doc = "0x14c - PMU Key Register"]
    pub pmukey: PMUKEY,
}
#[doc = "PMU Wake Program Memory"]
pub struct PMUWAKEPM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU Wake Program Memory"]
pub mod pmuwakepm;
#[doc = "PMU Sleep Program Memory"]
pub struct PMUSLEEPPM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU Sleep Program Memory"]
pub mod pmusleeppm;
#[doc = "PMU Interrupt Enable Register"]
pub struct PMUIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU Interrupt Enable Register"]
pub mod pmuie;
#[doc = "PMU Cause Register"]
pub struct PMUCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU Cause Register"]
pub mod pmucause;
#[doc = "PMU Sleep Register"]
pub struct PMUSLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU Sleep Register"]
pub mod pmusleep;
#[doc = "PMU Key Register"]
pub struct PMUKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMU Key Register"]
pub mod pmukey;
