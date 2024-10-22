#[doc = r" Core interrupts. These interrupts are handled by the core itself."]
# [riscv :: pac_enum (unsafe CoreInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreInterrupt {
    #[doc = "3 - Machine Software Interrupt"]
    MachineSoft = 3,
    #[doc = "7 - Machine Timer Interrupt"]
    MachineTimer = 7,
    #[doc = "11 - Machine External Interrupt"]
    MachineExternal = 11,
}
pub use riscv::interrupt::Exception;
#[doc = r" Priority levels in the device"]
# [riscv :: pac_enum (unsafe PriorityNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    #[doc = "0 - Priority level 0"]
    P0 = 0,
    #[doc = "1 - Priority level 1"]
    P1 = 1,
    #[doc = "2 - Priority level 2"]
    P2 = 2,
    #[doc = "3 - Priority level 3"]
    P3 = 3,
    #[doc = "4 - Priority level 4"]
    P4 = 4,
    #[doc = "5 - Priority level 5"]
    P5 = 5,
    #[doc = "6 - Priority level 6"]
    P6 = 6,
    #[doc = "7 - Priority level 7"]
    P7 = 7,
}
#[doc = r" HARTs in the device"]
# [riscv :: pac_enum (unsafe HartIdNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hart {
    #[doc = "0 - Hart 0"]
    H0 = 0,
}
pub use riscv::{
    interrupt::{disable, enable, free, nested},
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
#[doc = r" External interrupts. These interrupts are handled by the external peripherals."]
# [riscv :: pac_enum (unsafe ExternalInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalInterrupt {
    #[doc = "1 - WATCHDOG"]
    WATCHDOG = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - UART0"]
    UART0 = 3,
    #[doc = "4 - UART1"]
    UART1 = 4,
    #[doc = "5 - QSPI0"]
    QSPI0 = 5,
    #[doc = "6 - QSPI1"]
    QSPI1 = 6,
    #[doc = "7 - QSPI2"]
    QSPI2 = 7,
    #[doc = "8 - GPIO0"]
    GPIO0 = 8,
    #[doc = "9 - GPIO1"]
    GPIO1 = 9,
    #[doc = "10 - GPIO2"]
    GPIO2 = 10,
    #[doc = "11 - GPIO3"]
    GPIO3 = 11,
    #[doc = "12 - GPIO4"]
    GPIO4 = 12,
    #[doc = "13 - GPIO5"]
    GPIO5 = 13,
    #[doc = "14 - GPIO6"]
    GPIO6 = 14,
    #[doc = "15 - GPIO7"]
    GPIO7 = 15,
    #[doc = "16 - GPIO8"]
    GPIO8 = 16,
    #[doc = "17 - GPIO9"]
    GPIO9 = 17,
    #[doc = "18 - GPIO10"]
    GPIO10 = 18,
    #[doc = "19 - GPIO11"]
    GPIO11 = 19,
    #[doc = "20 - GPIO12"]
    GPIO12 = 20,
    #[doc = "21 - GPIO13"]
    GPIO13 = 21,
    #[doc = "22 - GPIO14"]
    GPIO14 = 22,
    #[doc = "23 - GPIO15"]
    GPIO15 = 23,
    #[doc = "24 - GPIO16"]
    GPIO16 = 24,
    #[doc = "25 - GPIO17"]
    GPIO17 = 25,
    #[doc = "26 - GPIO18"]
    GPIO18 = 26,
    #[doc = "27 - GPIO19"]
    GPIO19 = 27,
    #[doc = "28 - GPIO20"]
    GPIO20 = 28,
    #[doc = "29 - GPIO21"]
    GPIO21 = 29,
    #[doc = "30 - GPIO22"]
    GPIO22 = 30,
    #[doc = "31 - GPIO23"]
    GPIO23 = 31,
    #[doc = "32 - GPIO24"]
    GPIO24 = 32,
    #[doc = "33 - GPIO25"]
    GPIO25 = 33,
    #[doc = "34 - GPIO26"]
    GPIO26 = 34,
    #[doc = "35 - GPIO27"]
    GPIO27 = 35,
    #[doc = "36 - GPIO28"]
    GPIO28 = 36,
    #[doc = "37 - GPIO29"]
    GPIO29 = 37,
    #[doc = "38 - GPIO30"]
    GPIO30 = 38,
    #[doc = "39 - GPIO31"]
    GPIO31 = 39,
    #[doc = "40 - PWM0CMP0"]
    PWM0CMP0 = 40,
    #[doc = "41 - PWM0CMP1"]
    PWM0CMP1 = 41,
    #[doc = "42 - PWM0CMP2"]
    PWM0CMP2 = 42,
    #[doc = "43 - PWM0CMP3"]
    PWM0CMP3 = 43,
    #[doc = "44 - PWM1CMP0"]
    PWM1CMP0 = 44,
    #[doc = "45 - PWM1CMP1"]
    PWM1CMP1 = 45,
    #[doc = "46 - PWM1CMP2"]
    PWM1CMP2 = 46,
    #[doc = "47 - PWM1CMP3"]
    PWM1CMP3 = 47,
    #[doc = "48 - PWM2CMP0"]
    PWM2CMP0 = 48,
    #[doc = "49 - PWM2CMP1"]
    PWM2CMP1 = 49,
    #[doc = "50 - PWM2CMP2"]
    PWM2CMP2 = 50,
    #[doc = "51 - PWM2CMP3"]
    PWM2CMP3 = 51,
    #[doc = "52 - I2C0"]
    I2C0 = 52,
}
#[cfg(feature = "rt")]
#[riscv_rt::core_interrupt(CoreInterrupt::MachineExternal)]
fn plic_handler() {
    let claim = crate::PLIC::ctx(Hart::H0).claim();
    if let Some(s) = claim.claim::<ExternalInterrupt>() {
        unsafe { _dispatch_external_interrupt(s.number()) }
        claim.complete(s);
    }
}
