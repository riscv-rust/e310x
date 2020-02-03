use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "Coreplex Local Interrupts"]
pub struct CLINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLINT {}
impl CLINT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clint::RegisterBlock {
        0x0200_0000 as *const _
    }
}
impl Deref for CLINT {
    type Target = clint::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const plic::RegisterBlock {
        0x0c00_0000 as *const _
    }
}
impl Deref for PLIC {
    type Target = plic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog::RegisterBlock {
        0x1000_0000 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x1000_0000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aonclk::RegisterBlock {
        0x1000_0000 as *const _
    }
}
impl Deref for AONCLK {
    type Target = aonclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const backup::RegisterBlock {
        0x1000_0000 as *const _
    }
}
impl Deref for BACKUP {
    type Target = backup::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x1000_0000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prci::RegisterBlock {
        0x1000_8000 as *const _
    }
}
impl Deref for PRCI {
    type Target = prci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otp::RegisterBlock {
        0x1001_0000 as *const _
    }
}
impl Deref for OTP {
    type Target = otp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x1001_2000 as *const _
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x1001_3000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        0x1001_4000 as *const _
    }
}
impl Deref for QSPI0 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x1001_5000 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub mod pwm0;
#[doc = "Inter-Integrated Circuit Master Interface (FE310-G002 only)"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x1001_6000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit Master Interface (FE310-G002 only)"]
pub mod i2c0;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x1002_3000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI1 {}
impl QSPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        0x1002_4000 as *const _
    }
}
impl Deref for QSPI1 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI1::ptr() }
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x1002_5000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI2 {}
impl QSPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi0::RegisterBlock {
        0x1003_4000 as *const _
    }
}
impl Deref for QSPI2 {
    type Target = qspi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI2::ptr() }
    }
}
#[doc = "8-bit timer with 4 cmp"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x1003_5000 as *const _
    }
}
impl Deref for PWM2 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM2::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
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
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
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
