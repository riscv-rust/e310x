extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "Coreplex Local Interrupts"]
pub struct CLINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLINT {}
impl CLINT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clint::RegisterBlock {
        33554432 as *const _
    }
}
impl Deref for CLINT {
    type Target = clint::RegisterBlock;
    fn deref(&self) -> &clint::RegisterBlock {
        unsafe { &*CLINT::ptr() }
    }
}
#[doc = "Coreplex Local Interrupts"]
pub mod clint;
#[doc = "Platform Level Interrupt Control"]
pub struct PLIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PLIC {}
impl PLIC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const plic::RegisterBlock {
        201326592 as *const _
    }
}
impl Deref for PLIC {
    type Target = plic::RegisterBlock;
    fn deref(&self) -> &plic::RegisterBlock {
        unsafe { &*PLIC::ptr() }
    }
}
#[doc = "Platform Level Interrupt Control"]
pub mod plic;
#[doc = "Watchdog"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &wdog::RegisterBlock {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Watchdog"]
pub mod wdog;
#[doc = "Watchdog"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Watchdog"]
pub mod rtc;
#[doc = "Always-On Clock Configuration"]
pub struct AONCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AONCLK {}
impl AONCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aonclk::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for AONCLK {
    type Target = aonclk::RegisterBlock;
    fn deref(&self) -> &aonclk::RegisterBlock {
        unsafe { &*AONCLK::ptr() }
    }
}
#[doc = "Always-On Clock Configuration"]
pub mod aonclk;
#[doc = "Backup Registers"]
pub struct BACKUP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP {}
impl BACKUP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const backup::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for BACKUP {
    type Target = backup::RegisterBlock;
    fn deref(&self) -> &backup::RegisterBlock {
        unsafe { &*BACKUP::ptr() }
    }
}
#[doc = "Backup Registers"]
pub mod backup;
#[doc = "PMU"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    fn deref(&self) -> &pmu::RegisterBlock {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "PMU"]
pub mod pmu;
#[doc = "Power Reset Clock Interrupts"]
pub struct PRCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCI {}
impl PRCI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prci::RegisterBlock {
        268468224 as *const _
    }
}
impl Deref for PRCI {
    type Target = prci::RegisterBlock;
    fn deref(&self) -> &prci::RegisterBlock {
        unsafe { &*PRCI::ptr() }
    }
}
#[doc = "Power Reset Clock Interrupts"]
pub mod prci;
#[doc = "One Time Programmable Memory"]
pub struct OTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTP {}
impl OTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otp::RegisterBlock {
        268500992 as *const _
    }
}
impl Deref for OTP {
    type Target = otp::RegisterBlock;
    fn deref(&self) -> &otp::RegisterBlock {
        unsafe { &*OTP::ptr() }
    }
}
#[doc = "One Time Programmable Memory"]
pub mod otp;
#[doc = "General Purpose Input Output"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio0::RegisterBlock {
        268509184 as *const _
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    fn deref(&self) -> &gpio0::RegisterBlock {
        unsafe { &*GPIO0::ptr() }
    }
}
#[doc = "General Purpose Input Output"]
pub mod gpio0;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        268513280 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub mod uart0;
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI0 {}
impl QSPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qspi0::RegisterBlock {
        268517376 as *const _
    }
}
impl Deref for QSPI0 {
    type Target = qspi0::RegisterBlock;
    fn deref(&self) -> &qspi0::RegisterBlock {
        unsafe { &*QSPI0::ptr() }
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub mod qspi0;
#[doc = "8-bit timer with 4 cmp"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        268521472 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub mod pwm0;
#[doc = "Inter-Integrated Circuit Master Interface"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        268525568 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit Master Interface"]
pub mod i2c0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        268578816 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "QSPI1"]
pub struct QSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI1 {}
impl QSPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qspi0::RegisterBlock {
        268582912 as *const _
    }
}
impl Deref for QSPI1 {
    type Target = qspi0::RegisterBlock;
    fn deref(&self) -> &qspi0::RegisterBlock {
        unsafe { &*QSPI1::ptr() }
    }
}
#[doc = "PWM1"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        268587008 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "QSPI2"]
pub struct QSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI2 {}
impl QSPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qspi0::RegisterBlock {
        268648448 as *const _
    }
}
impl Deref for QSPI2 {
    type Target = qspi0::RegisterBlock;
    fn deref(&self) -> &qspi0::RegisterBlock {
        unsafe { &*QSPI2::ptr() }
    }
}
#[doc = "PWM2"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        268652544 as *const _
    }
}
impl Deref for PWM2 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM2::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
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
    #[doc = "I2C0"]
    pub I2C0: I2C0,
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
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CLINT: CLINT {
                _marker: PhantomData,
            },
            PLIC: PLIC {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            AONCLK: AONCLK {
                _marker: PhantomData,
            },
            BACKUP: BACKUP {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            PRCI: PRCI {
                _marker: PhantomData,
            },
            OTP: OTP {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            QSPI0: QSPI0 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            QSPI1: QSPI1 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            QSPI2: QSPI2 {
                _marker: PhantomData,
            },
            PWM2: PWM2 {
                _marker: PhantomData,
            },
        }
    }
}
