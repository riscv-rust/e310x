#![doc = "Peripheral access API for FE310 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;

mod common;

pub use common::{
    aonclk, backup, clint, gpio0, otp, plic, pmu, prci, pwm0, qspi0, rtc, uart0, wdog, AONCLK,
    BACKUP, CLINT, GPIO0, OTP, PLIC, PMU, PRCI, PWM0, PWM1, PWM2, QSPI0, QSPI1, QSPI2, RTC, UART0,
    UART1, WDOG, Interrupt,
};

#[doc(hidden)]
pub mod interrupt {
    pub use super::common::interrupt::Interrupt;
}

/// All the peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CLINT"]
    pub CLINT: CLINT,
    #[doc = "PLIC"]
    pub PLIC: PLIC,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "AONCLK"]
    pub AONCLK: AONCLK,
    #[doc = "BACKUP"]
    pub BACKUP: BACKUP,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "PRCI"]
    pub PRCI: PRCI,
    #[doc = "OTP"]
    pub OTP: OTP,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "QSPI0"]
    pub QSPI0: QSPI0,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "QSPI1"]
    pub QSPI1: QSPI1,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "QSPI2"]
    pub QSPI2: QSPI2,
    #[doc = "PWM2"]
    pub PWM2: PWM2,
}

impl Peripherals {
    #[inline]
    fn from_common(peripherals: common::Peripherals) -> Self {
        Self {
            CLINT: peripherals.CLINT,
            PLIC: peripherals.PLIC,
            WDOG: peripherals.WDOG,
            RTC: peripherals.RTC,
            AONCLK: peripherals.AONCLK,
            BACKUP: peripherals.BACKUP,
            PMU: peripherals.PMU,
            PRCI: peripherals.PRCI,
            OTP: peripherals.OTP,
            GPIO0: peripherals.GPIO0,
            UART0: peripherals.UART0,
            QSPI0: peripherals.QSPI0,
            PWM0: peripherals.PWM0,
            UART1: peripherals.UART1,
            QSPI1: peripherals.QSPI1,
            PWM1: peripherals.PWM1,
            QSPI2: peripherals.QSPI2,
            PWM2: peripherals.PWM2,
        }
    }

    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        common::Peripherals::take().map(Self::from_common)
    }

    /// Unchecked version of `Peripherals::take`
    pub unsafe fn steal() -> Self {
        Self::from_common(common::Peripherals::steal())
    }
}
