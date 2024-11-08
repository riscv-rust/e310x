//! # Pulse Width Modulation Interface
//!
//! You can use the `PWM` with these [Pwm] instances
//!
//! # PWM0 - 8 bit period and duty
//! - Channel 1: Pin 1 IOF1
//! - Channel 2: Pin 2 IOF1
//! - Channel 3: Pin 3 IOF1
//!
//! # PWM1 - 16 bit period and duty
//! - Channel 1: Pin 19 IOF1
//! - Channel 2: Pin 21 IOF1
//! - Channel 3: Pin 22 IOF1
//!
//! # PWM2 - 16 bit period and duty
//! - Channel 1: Pin 11 IOF1
//! - Channel 2: Pin 12 IOF1
//! - Channel 3: Pin 13 IOF1

use core::ops::Deref;
use e310x::{pwm0, Pwm0, Pwm1, Pwm2};
use embedded_hal::pwm::{ErrorKind, ErrorType, SetDutyCycle};

/// PWM comparator index
#[derive(Copy, Clone)]
pub enum CmpIndex {
    /// PWM comparator 1
    Cmp1,
    /// PWM comparator 1
    Cmp2,
    /// PWM comparator 1
    Cmp3,
}

/// PWM pin
pub trait Pin<PWM: PwmX>: private::Sealed {
    /// Channel index associated with the pin
    const CMP_INDEX: CmpIndex;
}

mod pwm_impl {
    use super::{CmpIndex, Pin, Pwm0, Pwm1, Pwm2};
    use crate::gpio::{gpio0, Invert, IOF1};

    // PWM0
    impl Pin<Pwm0> for gpio0::Pin1<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp1;
    }
    impl Pin<Pwm0> for gpio0::Pin2<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp2;
    }
    impl Pin<Pwm0> for gpio0::Pin3<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp3;
    }

    // PWM1
    impl Pin<Pwm1> for gpio0::Pin19<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp1;
    }
    impl Pin<Pwm1> for gpio0::Pin21<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp2;
    }
    impl Pin<Pwm1> for gpio0::Pin22<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp3;
    }

    // PWM2
    impl Pin<Pwm2> for gpio0::Pin11<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp1;
    }
    impl Pin<Pwm2> for gpio0::Pin12<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp2;
    }
    impl Pin<Pwm2> for gpio0::Pin13<IOF1<Invert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp3;
    }
}

/// PwmX trait extends the PWM peripherals
#[doc(hidden)]
pub trait PwmX: Deref<Target = pwm0::RegisterBlock> + private::Sealed {
    type CmpWidth: Ord;
    fn bits_from_cmp_width(other: Self::CmpWidth) -> u32;
    fn bits_into_cmp_width(other: u32) -> Self::CmpWidth;
}

macro_rules! pwmx_impl {
    ($PWM:ident,$CMP_WIDTH:ident) => {
        impl PwmX for $PWM {
            type CmpWidth = $CMP_WIDTH;
            fn bits_from_cmp_width(other: Self::CmpWidth) -> u32 {
                other as u32
            }
            fn bits_into_cmp_width(other: u32) -> Self::CmpWidth {
                other as Self::CmpWidth
            }
        }
    };
}

pwmx_impl!(Pwm0, u8);
pwmx_impl!(Pwm1, u16);
pwmx_impl!(Pwm2, u16);

/// PWM abstraction
///
/// # Notes
///
/// [PWM0] has a max period of 255, as it only has an 8 bit comparison register,
/// the rest of them have a max value of 2^16 as they have 16 bit registers.
pub struct Pwm<PWM> {
    pwm: PWM,
}

impl<PWM: PwmX> Pwm<PWM> {
    /// Configures a PWM device
    pub fn new(pwm: PWM) -> Self {
        pwm.cfg().reset();
        pwm.cfg().write(|w| {
            w.zerocmp()
                .set_bit()
                .enalways()
                .set_bit()
                .deglitch()
                .set_bit()
        });
        pwm.cmp0().reset();
        pwm.cmp1().reset();
        pwm.cmp2().reset();
        pwm.cmp3().reset();
        Self { pwm }
    }

    /// Frees the PWM device
    pub fn free(self) -> PWM {
        self.pwm
    }

    /// Returns the period of the PWM
    pub fn get_period(&self) -> PWM::CmpWidth {
        PWM::bits_into_cmp_width(self.pwm.cmp0().read().bits())
    }

    /// Sets the period of the PWM
    pub fn set_period(&mut self, period: PWM::CmpWidth) {
        let period = PWM::bits_from_cmp_width(period);
        self.pwm.count().reset();
        self.pwm.cmp0().write(|w| unsafe { w.bits(period) });
    }

    /// Returns the duty cycle of the PWM
    fn get_duty(&self, cmp_index: CmpIndex) -> PWM::CmpWidth {
        let duty = match cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().read().bits(),
            CmpIndex::Cmp2 => self.pwm.cmp2().read().bits(),
            CmpIndex::Cmp3 => self.pwm.cmp3().read().bits(),
        };
        PWM::bits_into_cmp_width(duty)
    }

    /// Sets the duty cycle of the PWM
    fn set_duty(&self, cmp_index: CmpIndex, duty: PWM::CmpWidth) {
        let duty = PWM::bits_from_cmp_width(duty.min(self.get_period()));
        match cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().write(|w| unsafe { w.bits(duty) }),
            CmpIndex::Cmp2 => self.pwm.cmp2().write(|w| unsafe { w.bits(duty) }),
            CmpIndex::Cmp3 => self.pwm.cmp3().write(|w| unsafe { w.bits(duty) }),
        };
    }

    /// Enables the PWM channel
    fn enable(&self, cmp_index: CmpIndex) {
        match cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().write(|w| unsafe { w.bits(u32::MAX) }),
            CmpIndex::Cmp2 => self.pwm.cmp2().write(|w| unsafe { w.bits(u32::MAX) }),
            CmpIndex::Cmp3 => self.pwm.cmp3().write(|w| unsafe { w.bits(u32::MAX) }),
        };
    }

    /// Disables the PWM channel
    fn disable(&self, cmp_index: CmpIndex) {
        match cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().reset(),
            CmpIndex::Cmp2 => self.pwm.cmp2().reset(),
            CmpIndex::Cmp3 => self.pwm.cmp3().reset(),
        }
    }

    /// Returns the PWM channel associated with the given index
    pub fn channel<PIN: Pin<PWM>>(&self, pin: PIN) -> Channel<'_, PWM, PIN> {
        Channel { pwm: self, pin }
    }
}

/// PWM channel
pub struct Channel<'a, PWM, PIN> {
    pwm: &'a Pwm<PWM>,
    pin: PIN,
}

impl<'a, PWM, PIN> Channel<'a, PWM, PIN> {
    /// Frees the PWM channel
    pub fn free(self) -> PIN {
        self.pin
    }
}

impl<'a, PWM: PwmX, PIN: Pin<PWM>> Channel<'a, PWM, PIN> {
    /// Returns the period of the PWM
    pub fn get_period(&self) -> PWM::CmpWidth {
        self.pwm.get_period()
    }

    /// Returns the maximum duty cycle of the PWM (equal to the period)
    pub fn max_duty(&self) -> PWM::CmpWidth {
        self.pwm.get_period()
    }

    /// Returns the duty cycle of the PWM
    pub fn get_duty(&self) -> PWM::CmpWidth {
        self.pwm.get_duty(PIN::CMP_INDEX)
    }

    /// Sets the duty cycle to 100%
    pub fn enable(&mut self) {
        self.pwm.enable(PIN::CMP_INDEX);
    }

    /// Sets the duty cycle to 0%
    pub fn disable(&mut self) {
        self.pwm.disable(PIN::CMP_INDEX);
    }

    /// Sets the duty cycle of the PWM
    pub fn set_duty(&mut self, duty: PWM::CmpWidth) {
        self.pwm.set_duty(PIN::CMP_INDEX, duty);
    }
}

impl<'a, PWM: PwmX, PIN: Pin<PWM>> ErrorType for Channel<'a, PWM, PIN> {
    type Error = ErrorKind;
}

impl<'a, PWM: PwmX, PIN: Pin<PWM>> SetDutyCycle for Channel<'a, PWM, PIN> {
    fn max_duty_cycle(&self) -> u16 {
        PWM::bits_from_cmp_width(self.max_duty()) as _
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.pwm
            .set_duty(PIN::CMP_INDEX, PWM::bits_into_cmp_width(duty as _));
        Ok(())
    }
}

mod private {
    use super::{Pwm0, Pwm1, Pwm2};
    use crate::gpio::{gpio0, Invert, IOF1};
    pub trait Sealed {}

    // PWM0
    impl Sealed for Pwm0 {}
    impl Sealed for gpio0::Pin1<IOF1<Invert>> {}
    impl Sealed for gpio0::Pin2<IOF1<Invert>> {}
    impl Sealed for gpio0::Pin3<IOF1<Invert>> {}

    // PWM1
    impl Sealed for Pwm1 {}
    impl Sealed for gpio0::Pin19<IOF1<Invert>> {}
    impl Sealed for gpio0::Pin21<IOF1<Invert>> {}
    impl Sealed for gpio0::Pin22<IOF1<Invert>> {}

    // PWM2
    impl Sealed for Pwm2 {}
    impl Sealed for gpio0::Pin11<IOF1<Invert>> {}
    impl Sealed for gpio0::Pin12<IOF1<Invert>> {}
    impl Sealed for gpio0::Pin13<IOF1<Invert>> {}
}
