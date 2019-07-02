//! Device resources

use e310x::{
    CLINT, PLIC, WDOG, RTC, AONCLK, BACKUP, PMU, PRCI, OTP, UART0, PWM0, QSPI1, PWM1, PWM2, QSPI0
};
#[cfg(feature = "g002")]
use e310x::{I2C0, UART1};
use crate::gpio::{GpioExt, gpio0};

/// All the peripherals except GPIO0 and QSPI2
#[allow(non_snake_case)]
pub struct Peripherals {
    /// CLINT peripheral
    pub CLINT: CLINT,
    /// PLIC peripheral
    pub PLIC: PLIC,
    /// WDOG peripheral
    pub WDOG: WDOG,
    /// RTC peripheral
    pub RTC: RTC,
    /// AONCLK peripheral
    pub AONCLK: AONCLK,
    /// BACKUP peripheral
    pub BACKUP: BACKUP,
    /// PMU peripheral
    pub PMU: PMU,
    /// PRCI peripheral
    pub PRCI: PRCI,
    /// OTP peripheral
    pub OTP: OTP,

    /// UART0 peripheral
    pub UART0: UART0,
    #[cfg(feature = "g002")]
    /// UART1 peripheral
    pub UART1: UART1,

    /// QSPI0 peripheral
    pub QSPI0: QSPI0,
    /// QSPI1 peripheral
    pub QSPI1: QSPI1,

    #[cfg(feature = "g002")]
    /// I2C0 peripheral
    pub I2C0: I2C0,

    /// PWM0 peripheral
    pub PWM0: PWM0,
    /// PWM1 peripheral
    pub PWM1: PWM1,
    /// PWM2 peripheral
    pub PWM2: PWM2,
}

/// Device resources
pub struct DeviceResources {
    /// Device peripherals
    pub peripherals: Peripherals,

    /// Device pins
    pub pins: gpio0::Parts,
}

impl DeviceResources {
    fn construct(p: e310x::Peripherals) -> Self {
        let peripherals = Peripherals {
            CLINT: p.CLINT,
            PLIC: p.PLIC,
            WDOG: p.WDOG,
            RTC: p.RTC,
            AONCLK: p.AONCLK,
            BACKUP: p.BACKUP,
            PMU: p.PMU,
            PRCI: p.PRCI,
            OTP: p.OTP,

            UART0: p.UART0,
            #[cfg(feature = "g002")]
            UART1: p.UART1,

            QSPI0: p.QSPI0,
            QSPI1: p.QSPI1,

            #[cfg(feature = "g002")]
            I2C0: p.I2C0,

            PWM0: p.PWM0,
            PWM1: p.PWM1,
            PWM2: p.PWM2,
        };

        DeviceResources {
            peripherals,
            pins: p.GPIO0.split(),
        }
    }

    /// Returns all the device resources *once*
    #[inline]
    pub fn take() -> Option<Self> {
        e310x::Peripherals::take().map(DeviceResources::construct)
    }

    /// Unchecked version of `DeviceResources::take`
    pub unsafe fn steal() -> Self {
        DeviceResources::construct(e310x::Peripherals::steal())
    }
}
