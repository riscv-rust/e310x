//! Device resources available in FE310-G000 and FE310-G002 chip packages

use crate::core::CorePeripherals;
use crate::gpio::{gpio0::*, EventType, GpioExt, Unknown};
use e310x::{
    Aonclk, Backup, Gpio0, Otp, Peripherals, Pmu, Prci, Pwm0, Pwm1, Pwm2, Qspi0, Qspi1, Rtc, Uart0,
    Wdog,
};
#[cfg(feature = "g002")]
use e310x::{I2c0, Uart1};

/// Device peripherals available in a 48QFN package, except GPIO0
#[allow(non_snake_case)]
pub struct DevicePeripherals {
    /// WDOG peripheral
    pub WDOG: Wdog,
    /// RTC peripheral
    pub RTC: Rtc,
    /// AONCLK peripheral
    pub AONCLK: Aonclk,
    /// BACKUP peripheral
    pub BACKUP: Backup,
    /// PMU peripheral
    pub PMU: Pmu,
    /// PRCI peripheral
    pub PRCI: Prci,
    /// OTP peripheral
    pub OTP: Otp,

    /// UART0 peripheral
    pub UART0: Uart0,
    #[cfg(feature = "g002")]
    /// UART1 peripheral (FE310-G002 only)
    pub UART1: Uart1,

    /// QSPI0 peripheral
    pub QSPI0: Qspi0,
    /// QSPI1 peripheral
    pub QSPI1: Qspi1,

    #[cfg(feature = "g002")]
    /// I2C0 peripheral (FE310-G002 only)
    pub I2C0: I2c0,

    /// PWM0 peripheral
    pub PWM0: Pwm0,
    /// PWM1 peripheral
    pub PWM1: Pwm1,
    /// PWM2 peripheral
    pub PWM2: Pwm2,
}

/// Device GPIO pins available in a 48QFN package
pub struct DeviceGpioPins {
    /// GPIO 0, package pin 25
    pub pin0: Pin0<Unknown>,
    /// GPIO 1, package pin 26
    pub pin1: Pin1<Unknown>,
    /// GPIO 2, package pin 27
    pub pin2: Pin2<Unknown>,
    /// GPIO 3, package pin 28
    pub pin3: Pin3<Unknown>,
    /// GPIO 4, package pin 29
    pub pin4: Pin4<Unknown>,
    /// GPIO 5, package pin 31
    pub pin5: Pin5<Unknown>,
    /// GPIO 9, package pin 33
    pub pin9: Pin9<Unknown>,
    /// GPIO 10, package pin 34
    pub pin10: Pin10<Unknown>,
    /// GPIO 11, package pin 35
    pub pin11: Pin11<Unknown>,
    /// GPIO 12, package pin 36
    pub pin12: Pin12<Unknown>,
    /// GPIO 13, package pin 37
    pub pin13: Pin13<Unknown>,
    /// GPIO 16, package pin 38
    pub pin16: Pin16<Unknown>,
    /// GPIO 17, package pin 39
    pub pin17: Pin17<Unknown>,
    /// GPIO 18, package pin 40
    pub pin18: Pin18<Unknown>,
    /// GPIO 19, package pin 41
    pub pin19: Pin19<Unknown>,
    /// GPIO 20, package pin 42
    pub pin20: Pin20<Unknown>,
    /// GPIO 21, package pin 43
    pub pin21: Pin21<Unknown>,
    /// GPIO 22, package pin 44
    pub pin22: Pin22<Unknown>,
    /// GPIO 23, package pin 45
    pub pin23: Pin23<Unknown>,
}

impl DeviceGpioPins {
    /// Enables the specified interrupt event for all the GPIO pins.
    ///
    /// # Note
    ///
    ///  This function does not enable the interrupts in the PLIC, it only sets the
    /// interrupt enable bits in the GPIO peripheral. You must call the
    /// [`enable_exti()`](super::gpio::gpio0::Pin0::enable_exti) method of every pin
    /// to enable their interrupt in the PLIC.
    pub fn enable_interrupts(&self, event: EventType) {
        let gpio = unsafe { Gpio0::steal() };

        match event {
            EventType::High => {
                unsafe { gpio.high_ie().write(|w| w.bits(0xFFFFFFFF)) };
            }
            EventType::Low => {
                unsafe { gpio.low_ie().write(|w| w.bits(0xFFFFFFFF)) };
            }
            EventType::BothLevels => unsafe {
                gpio.high_ie().write(|w| w.bits(0xFFFFFFFF));
                gpio.low_ie().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::Rise => {
                unsafe { gpio.rise_ie().write(|w| w.bits(0xFFFFFFFF)) };
            }
            EventType::Fall => {
                unsafe { gpio.fall_ie().write(|w| w.bits(0xFFFFFFFF)) };
            }
            EventType::BothEdges => unsafe {
                gpio.rise_ie().write(|w| w.bits(0xFFFFFFFF));
                gpio.fall_ie().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::All => unsafe {
                gpio.high_ie().write(|w| w.bits(0xFFFFFFFF));
                gpio.low_ie().write(|w| w.bits(0xFFFFFFFF));
                gpio.rise_ie().write(|w| w.bits(0xFFFFFFFF));
                gpio.fall_ie().write(|w| w.bits(0xFFFFFFFF));
            },
        }
    }

    /// Disables the specified interrupt event for all the GPIO pins.
    pub fn disable_interrupts(&self, event: EventType) {
        let gpio = unsafe { Gpio0::steal() };

        match event {
            EventType::High => unsafe {
                gpio.high_ie().write(|w| w.bits(0x00000000));
            },
            EventType::Low => unsafe {
                gpio.low_ie().write(|w| w.bits(0x00000000));
            },
            EventType::BothLevels => unsafe {
                gpio.high_ie().write(|w| w.bits(0x00000000));
                gpio.low_ie().write(|w| w.bits(0x00000000));
            },
            EventType::Rise => unsafe {
                gpio.rise_ie().write(|w| w.bits(0x00000000));
            },
            EventType::Fall => unsafe {
                gpio.fall_ie().write(|w| w.bits(0x00000000));
            },
            EventType::BothEdges => unsafe {
                gpio.rise_ie().write(|w| w.bits(0x00000000));
                gpio.fall_ie().write(|w| w.bits(0x00000000));
            },
            EventType::All => unsafe {
                gpio.high_ie().write(|w| w.bits(0x00000000));
                gpio.low_ie().write(|w| w.bits(0x00000000));
                gpio.rise_ie().write(|w| w.bits(0x00000000));
                gpio.fall_ie().write(|w| w.bits(0x00000000));
            },
        }
    }

    /// Clears the specified interrupt event pending flag for all the GPIO pins.
    pub fn clear_interrupts(&self, event: EventType) {
        let gpio = unsafe { Gpio0::steal() };

        match event {
            EventType::High => unsafe {
                gpio.high_ip().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::Low => unsafe {
                gpio.low_ip().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::BothLevels => unsafe {
                gpio.high_ip().write(|w| w.bits(0xFFFFFFFF));
                gpio.low_ip().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::Rise => unsafe {
                gpio.rise_ip().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::Fall => unsafe {
                gpio.fall_ip().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::BothEdges => unsafe {
                gpio.rise_ip().write(|w| w.bits(0xFFFFFFFF));
                gpio.fall_ip().write(|w| w.bits(0xFFFFFFFF));
            },
            EventType::All => unsafe {
                gpio.high_ip().write(|w| w.bits(0xFFFFFFFF));
                gpio.low_ip().write(|w| w.bits(0xFFFFFFFF));
                gpio.rise_ip().write(|w| w.bits(0xFFFFFFFF));
                gpio.fall_ip().write(|w| w.bits(0xFFFFFFFF));
            },
        }
    }
}

impl From<Gpio0> for DeviceGpioPins {
    fn from(gpio: Gpio0) -> Self {
        let parts = gpio.split();

        DeviceGpioPins {
            pin0: parts.pin0,
            pin1: parts.pin1,
            pin2: parts.pin2,
            pin3: parts.pin3,
            pin4: parts.pin4,
            pin5: parts.pin5,
            pin9: parts.pin9,
            pin10: parts.pin10,
            pin11: parts.pin11,
            pin12: parts.pin12,
            pin13: parts.pin13,
            pin16: parts.pin16,
            pin17: parts.pin17,
            pin18: parts.pin18,
            pin19: parts.pin19,
            pin20: parts.pin20,
            pin21: parts.pin21,
            pin22: parts.pin22,
            pin23: parts.pin23,
        }
    }
}

/// Device resources available in a 48QFN package
pub struct DeviceResources {
    /// Core peripherals
    pub core_peripherals: CorePeripherals,

    /// Device peripherals
    pub peripherals: DevicePeripherals,

    /// Device GPIO pins
    pub pins: DeviceGpioPins,
}

impl From<Peripherals> for DeviceResources {
    fn from(p: Peripherals) -> Self {
        let peripherals = DevicePeripherals {
            WDOG: p.wdog,
            RTC: p.rtc,
            AONCLK: p.aonclk,
            BACKUP: p.backup,
            PMU: p.pmu,
            PRCI: p.prci,
            OTP: p.otp,

            UART0: p.uart0,
            #[cfg(feature = "g002")]
            UART1: p.uart1,

            QSPI0: p.qspi0,
            QSPI1: p.qspi1,

            #[cfg(feature = "g002")]
            I2C0: p.i2c0,

            PWM0: p.pwm0,
            PWM1: p.pwm1,
            PWM2: p.pwm2,
        };

        DeviceResources {
            core_peripherals: CorePeripherals::new(p.clint, p.plic),
            peripherals,
            pins: p.gpio0.into(),
        }
    }
}

impl DeviceResources {
    /// Returns all the device resources *once*
    #[inline]
    pub fn take() -> Option<Self> {
        e310x::Peripherals::take().map(DeviceResources::from)
    }

    /// Unchecked version of [`DeviceResources::take`].
    ///
    /// # Safety
    ///
    /// Using this function may break the guarantees of the singleton pattern.
    pub unsafe fn steal() -> Self {
        e310x::Peripherals::steal().into()
    }
}
