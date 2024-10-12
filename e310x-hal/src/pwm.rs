//! # Pulse Width Modulation Interface
//!
//! You can use the `PWM` with these [Pwm] instances
//!
//! # PWM0 - 8 bit period and duty
//! - Channel 1: Pin 9 IOF1
//! - Channel 2: Pin 10 IOF1
//! - Channel 3: Pin 11 IOF1
//!
//! # PWM1 - 16 bit period and duty
//! - Channel 1: Pin 3 IOF1
//! - Channel 2: Pin 5 IOF1
//! - Channel 3: Pin 6 IOF1
//!
//! # PWM2 - 16 bit period and duty
//! - Channel 1: Pin 17 IOF1
//! - Channel 2: Pin 18 IOF1
//! - Channel 3: Pin 19 IOF1

use core::{marker::PhantomData, ops::Deref};

use e310x::{pwm0, Pwm0, Pwm1, Pwm2};

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
pub trait Pin<PWM>: private::Sealed {
    /// Channel index associated with the pin
    const CMP_INDEX: CmpIndex;
}

mod pwm_impl {
    use super::{CmpIndex, Pin, Pwm0, Pwm1, Pwm2};
    use crate::gpio::{gpio0, NoInvert, IOF1};

    // PWM0
    impl Pin<Pwm0> for gpio0::Pin1<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp1;
    }
    impl Pin<Pwm0> for gpio0::Pin2<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp2;
    }
    impl Pin<Pwm0> for gpio0::Pin3<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp3;
    }

    // PWM1
    impl Pin<Pwm1> for gpio0::Pin19<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp1;
    }
    impl Pin<Pwm1> for gpio0::Pin21<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp2;
    }
    impl Pin<Pwm1> for gpio0::Pin22<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp3;
    }

    // PWM2
    impl Pin<Pwm2> for gpio0::Pin11<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp1;
    }
    impl Pin<Pwm2> for gpio0::Pin12<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp2;
    }
    impl Pin<Pwm2> for gpio0::Pin13<IOF1<NoInvert>> {
        const CMP_INDEX: CmpIndex = CmpIndex::Cmp3;
    }
}

/// PWM channel
#[derive(Copy, Clone)]
pub struct Channel<PWM> {
    _pwm: PhantomData<PWM>,
    cmp_index: CmpIndex,
}

impl<PWM> Channel<PWM> {
    /// Constructs a PWM channel from a PWM pin for use with [Pwm]
    pub fn from<PIN: Pin<PWM>>(_: PIN) -> Channel<PWM> {
        Channel {
            _pwm: PhantomData,
            cmp_index: PIN::CMP_INDEX,
        }
    }
}

/// PwmX trait extends the PWM peripherals
#[doc(hidden)]
pub trait PwmX: Deref<Target = pwm0::RegisterBlock> + private::Sealed {
    type CmpWidth: Ord;
    fn peripheral() -> Self;
    fn bits_from_cmp_width(other: Self::CmpWidth) -> u32;
    fn bits_into_cmp_width(other: u32) -> Self::CmpWidth;
}

macro_rules! pwmx_impl {
    ($PWM:ident,$CMP_WIDTH:ident) => {
        impl PwmX for $PWM {
            type CmpWidth = $CMP_WIDTH;
            fn peripheral() -> Self {
                unsafe { Self::steal() }
            }
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

    /// Enables the PWM channel
    pub fn enable(&mut self, channel: &Channel<PWM>) {
        match channel.cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().write(|w| unsafe { w.bits(u32::MAX) }),
            CmpIndex::Cmp2 => self.pwm.cmp2().write(|w| unsafe { w.bits(u32::MAX) }),
            CmpIndex::Cmp3 => self.pwm.cmp3().write(|w| unsafe { w.bits(u32::MAX) }),
        }
    }

    /// Disables the PWM channel
    pub fn disable(&mut self, channel: &Channel<PWM>) {
        match channel.cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().reset(),
            CmpIndex::Cmp2 => self.pwm.cmp2().reset(),
            CmpIndex::Cmp3 => self.pwm.cmp3().reset(),
        }
    }

    /// Returns the period of the PWM
    pub fn get_period(&self) -> PWM::CmpWidth {
        PWM::bits_into_cmp_width(self.pwm.cmp0().read().bits())
    }

    /// Returns the duty cycle of the PWM
    pub fn get_duty(&self, channel: &Channel<PWM>) -> PWM::CmpWidth {
        let duty = match channel.cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().read().bits(),
            CmpIndex::Cmp2 => self.pwm.cmp2().read().bits(),
            CmpIndex::Cmp3 => self.pwm.cmp3().read().bits(),
        };
        PWM::bits_into_cmp_width(duty)
    }

    /// Returns the maximum duty cycle of the PWM (equal to the period)
    pub fn get_max_duty(&self) -> PWM::CmpWidth {
        self.get_period()
    }

    /// Sets the duty cycle of the PWM
    pub fn set_duty(&mut self, channel: &Channel<PWM>, duty: PWM::CmpWidth) {
        let duty = PWM::bits_from_cmp_width(duty.min(self.get_max_duty()));
        match channel.cmp_index {
            CmpIndex::Cmp1 => self.pwm.cmp1().write(|w| unsafe { w.bits(duty) }),
            CmpIndex::Cmp2 => self.pwm.cmp2().write(|w| unsafe { w.bits(duty) }),
            CmpIndex::Cmp3 => self.pwm.cmp3().write(|w| unsafe { w.bits(duty) }),
        }
    }

    /// Sets the period of the PWM
    pub fn set_period(&mut self, period: PWM::CmpWidth) {
        let period = PWM::bits_from_cmp_width(period);
        self.pwm.count().reset();
        self.pwm.cmp0().write(|w| unsafe { w.bits(period) });
    }
}

impl<PWM: PwmX> embedded_hal::pwm::ErrorType for Channel<PWM> {
    type Error = embedded_hal::pwm::ErrorKind;
}

impl<PWM: PwmX> embedded_hal::pwm::SetDutyCycle for Channel<PWM> {
    fn max_duty_cycle(&self) -> u16 {
        let pwm: Pwm<PWM> = Pwm {
            pwm: PWM::peripheral(),
        };
        PWM::bits_from_cmp_width(pwm.get_max_duty()) as _
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let mut pwm: Pwm<PWM> = Pwm {
            pwm: PWM::peripheral(),
        };
        pwm.set_duty(self, PWM::bits_into_cmp_width(duty as _));
        Ok(())
    }
}

mod private {
    use super::{Pwm0, Pwm1, Pwm2};
    use crate::gpio::{gpio0, NoInvert, IOF1};
    pub trait Sealed {}

    // PWM0
    impl Sealed for Pwm0 {}
    impl Sealed for gpio0::Pin1<IOF1<NoInvert>> {}
    impl Sealed for gpio0::Pin2<IOF1<NoInvert>> {}
    impl Sealed for gpio0::Pin3<IOF1<NoInvert>> {}

    // PWM1
    impl Sealed for Pwm1 {}
    impl Sealed for gpio0::Pin19<IOF1<NoInvert>> {}
    impl Sealed for gpio0::Pin21<IOF1<NoInvert>> {}
    impl Sealed for gpio0::Pin22<IOF1<NoInvert>> {}

    // PWM2
    impl Sealed for Pwm2 {}
    impl Sealed for gpio0::Pin11<IOF1<NoInvert>> {}
    impl Sealed for gpio0::Pin12<IOF1<NoInvert>> {}
    impl Sealed for gpio0::Pin13<IOF1<NoInvert>> {}
}
