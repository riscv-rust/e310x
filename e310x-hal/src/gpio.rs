//! General Purpose I/O

use core::marker::PhantomData;

#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicU32, Ordering};
#[cfg(not(target_has_atomic = "32"))]
use portable_atomic::{AtomicU32, Ordering};

/// GpioExt trait extends the GPIO0 peripheral.
pub trait GpioExt {
    /// The parts to split the GPIO into.
    type Parts;

    /// Splits the GPIO block into independent pins and registers.
    fn split(self) -> Self::Parts;
}

/// Unknown mode (type state)
pub struct Unknown;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Regular output mode (type state)
pub struct Regular<INVERT> {
    _mode: PhantomData<INVERT>,
}

/// High current mode (type state)
pub struct Drive<INVERT> {
    _mode: PhantomData<INVERT>,
}

/// Alternate function 0 (type state)
pub struct IOF0<INVERT> {
    _mode: PhantomData<INVERT>,
}

/// Alternate function 1 (type state)
pub struct IOF1<INVERT> {
    _mode: PhantomData<INVERT>,
}

/// Non-inverted output mode (type state)
pub struct NoInvert;

/// Invert output mode (type state)
pub struct Invert;

trait PinIndex {
    const INDEX: usize;
}

#[inline(always)]
fn atomic_set_bit(r: &AtomicU32, index: usize, bit: bool) {
    let mask = 1 << (index & 31);
    match bit {
        true => r.fetch_or(mask, Ordering::SeqCst),
        false => r.fetch_and(!mask, Ordering::SeqCst),
    };
}

trait PeripheralAccess {
    fn peripheral() -> &'static e310x::gpio0::RegisterBlock;

    fn input_value(index: usize) -> bool {
        let p = Self::peripheral();
        (p.input_val.read().bits() >> (index & 31) & 1) != 0
    }

    fn set_input_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.input_en) };
        atomic_set_bit(r, index, bit);
    }

    fn set_output_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.output_en) };
        atomic_set_bit(r, index, bit);
    }

    fn set_output_value(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.output_val) };
        atomic_set_bit(r, index, bit);
    }

    fn toggle_pin(index: usize) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.output_val) };
        let mask = 1 << (index & 31);
        r.fetch_xor(mask, Ordering::SeqCst);
    }

    fn set_pullup(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.pullup) };
        atomic_set_bit(r, index, bit);
    }

    fn set_drive(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.drive) };
        atomic_set_bit(r, index, bit);
    }

    fn set_out_xor(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.out_xor) };
        atomic_set_bit(r, index, bit);
    }

    fn set_iof_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.iof_en) };
        atomic_set_bit(r, index, bit);
    }

    fn set_iof_sel(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.iof_sel) };
        atomic_set_bit(r, index, bit);
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;
            use core::convert::Infallible;

            use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin,
                               ToggleableOutputPin};
            use e310x::$GPIOX;
            use super::{Unknown, IOF0, IOF1, Drive, Floating, GpioExt, Input, Invert,
                        NoInvert, Output, PullUp, Regular, PinIndex, PeripheralAccess};

            /// GPIO parts for fine grained permission control.
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl PeripheralAccess for $GPIOX {
                #[inline(always)]
                fn peripheral() -> &'static e310x::gpio0::RegisterBlock {
                    unsafe { &*$GPIOX::ptr() }
                }
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> PinIndex for $PXi<MODE> {
                    const INDEX: usize = $i;
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to serve as alternate function 0 (AF0)
                    pub fn into_iof0(self) -> $PXi<IOF0<NoInvert>> {
                        $GPIOX::set_out_xor(Self::INDEX, false);
                        $GPIOX::set_iof_sel(Self::INDEX, false);
                        $GPIOX::set_iof_en(Self::INDEX, true);
                        $PXi { _mode: PhantomData }
                    }


                    /// Configures the pin to serve as alternate function 1 (AF1)
                    pub fn into_iof1(self) -> $PXi<IOF1<NoInvert>> {
                        $GPIOX::set_out_xor(Self::INDEX, false);
                        $GPIOX::set_iof_sel(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, true);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as inverted alternate function 0 (AF0)
                    pub fn into_inverted_iof0(self) -> $PXi<IOF0<Invert>> {
                        $GPIOX::set_out_xor(Self::INDEX, true);
                        $GPIOX::set_iof_sel(Self::INDEX, false);
                        $GPIOX::set_iof_en(Self::INDEX, true);
                        $PXi { _mode: PhantomData }
                    }


                    /// Configures the pin to serve as inverted alternate function 1 (AF1)
                    pub fn into_inverted_iof1(self) -> $PXi<IOF1<Invert>> {
                        $GPIOX::set_out_xor(Self::INDEX, true);
                        $GPIOX::set_iof_sel(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, true);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as a floating input pin
                    pub fn into_floating_input(self) -> $PXi<Input<Floating>> {
                        $GPIOX::set_pullup(Self::INDEX, false);
                        $GPIOX::set_input_en(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, false);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_up_input(self) -> $PXi<Input<PullUp>> {
                        $GPIOX::set_pullup(Self::INDEX, true);
                        $GPIOX::set_input_en(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, false);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an output pin
                    pub fn into_output(self) -> $PXi<Output<Regular<NoInvert>>> {
                        $GPIOX::set_drive(Self::INDEX, false);
                        $GPIOX::set_out_xor(Self::INDEX, false);
                        $GPIOX::set_output_en(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, false);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an inverted output pin
                    pub fn into_inverted_output(self) -> $PXi<Output<Regular<Invert>>> {
                        $GPIOX::set_drive(Self::INDEX, false);
                        $GPIOX::set_out_xor(Self::INDEX, true);
                        $GPIOX::set_output_en(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, false);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configure the pin to operate as an output pin with high
                    /// current drive
                    pub fn into_output_drive(self) -> $PXi<Output<Drive<NoInvert>>> {
                        $GPIOX::set_drive(Self::INDEX, true);
                        $GPIOX::set_out_xor(Self::INDEX, false);
                        $GPIOX::set_output_en(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, false);
                        $PXi { _mode: PhantomData }
                    }

                    /// Configure the pin to operate as an inverted output pin with
                    /// high current drive
                    pub fn into_inverted_output_drive(self) -> $PXi<Output<Drive<Invert>>> {
                        $GPIOX::set_drive(Self::INDEX, true);
                        $GPIOX::set_out_xor(Self::INDEX, true);
                        $GPIOX::set_output_en(Self::INDEX, true);
                        $GPIOX::set_iof_en(Self::INDEX, false);
                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    type Error = Infallible;

                    fn is_high(&self) -> Result<bool, Infallible> {
                        Ok($GPIOX::input_value(Self::INDEX))

                    }

                    fn is_low(&self) -> Result<bool, Infallible> {
                        Ok(!self.is_high()?)
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    fn is_set_high(&self) -> Result<bool, Infallible> {
                        Ok($GPIOX::input_value(Self::INDEX))
                    }

                    fn is_set_low(&self) -> Result<bool, Infallible> {
                        Ok(!self.is_set_high()?)
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;

                    fn set_high(&mut self) -> Result<(), Infallible> {
                        $GPIOX::set_output_value(Self::INDEX, true);
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Infallible> {
                        $GPIOX::set_output_value(Self::INDEX, false);
                        Ok(())
                    }
                }

                impl<MODE> ToggleableOutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;

                    /// Toggles the pin state.
                    fn toggle(&mut self) -> Result<(), Infallible> {
                        $GPIOX::toggle_pin(Self::INDEX);
                        Ok(())
                    }
                }
            )+
        }
    }
}

// By default, all GPIOs are in the Unknown state for two reasons:
// * bootloader may reconfigure some GPIOs
// * we do not enforce any specific state in `split()`
gpio!(GPIO0, gpio0, [
    Pin0: (pin0, 0, Unknown),
    Pin1: (pin1, 1, Unknown),
    Pin2: (pin2, 2, Unknown),
    Pin3: (pin3, 3, Unknown),
    Pin4: (pin4, 4, Unknown),
    Pin5: (pin5, 5, Unknown),
    Pin6: (pin6, 6, Unknown),
    Pin7: (pin7, 7, Unknown),
    Pin8: (pin8, 8, Unknown),
    Pin9: (pin9, 9, Unknown),
    Pin10: (pin10, 10, Unknown),
    Pin11: (pin11, 11, Unknown),
    Pin12: (pin12, 12, Unknown),
    Pin13: (pin13, 13, Unknown),
    Pin14: (pin14, 14, Unknown),
    Pin15: (pin15, 15, Unknown),
    Pin16: (pin16, 16, Unknown),
    Pin17: (pin17, 17, Unknown),
    Pin18: (pin18, 18, Unknown),
    Pin19: (pin19, 19, Unknown),
    Pin20: (pin20, 20, Unknown),
    Pin21: (pin21, 21, Unknown),
    Pin22: (pin22, 22, Unknown),
    Pin23: (pin23, 23, Unknown),
    Pin24: (pin24, 24, Unknown),
    Pin25: (pin25, 25, Unknown),
    Pin26: (pin26, 26, Unknown),
    Pin27: (pin27, 27, Unknown),
    Pin28: (pin28, 28, Unknown),
    Pin29: (pin29, 29, Unknown),
    Pin30: (pin30, 30, Unknown),
    Pin31: (pin31, 31, Unknown),
]);
