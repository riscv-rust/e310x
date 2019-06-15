//! General Purpose I/O

use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, Ordering};

/// GpioExt trait extends the GPIO0 peripheral.
pub trait GpioExt {
    /// The parts to split the GPIO into.
    type Parts;

    /// Splits the GPIO block into independent pins and registers.
    fn split(self) -> Self::Parts;
}


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

    fn value(index: usize) -> bool {
        let p = Self::peripheral();
        (p.value.read().bits() >> (index & 31)) != 0
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

    fn set_port(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.port) };
        atomic_set_bit(r, index, bit);
    }

    fn toggle_port(index: usize) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(&p.port) };
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
            use super::{IOF0, IOF1, Drive, Floating, GpioExt, Input, Invert,
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
                        Ok($GPIOX::value(Self::INDEX))

                    }

                    fn is_low(&self) -> Result<bool, Infallible> {
                        Ok(!self.is_high()?)
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    fn is_set_high(&self) -> Result<bool, Infallible> {
                        Ok($GPIOX::value(Self::INDEX))
                    }

                    fn is_set_low(&self) -> Result<bool, Infallible> {
                        Ok(!self.is_set_high()?)
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;

                    fn set_high(&mut self) -> Result<(), Infallible> {
                        $GPIOX::set_port(Self::INDEX, true);
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Infallible> {
                        $GPIOX::set_port(Self::INDEX, false);
                        Ok(())
                    }
                }

                impl<MODE> ToggleableOutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;

                    /// Toggles the pin state.
                    fn toggle(&mut self) -> Result<(), Infallible> {
                        $GPIOX::toggle_port(Self::INDEX);
                        Ok(())
                    }
                }
            )+
        }
    }
}

gpio!(GPIO0, gpio0, [
    Pin0: (pin0, 0, Input<Floating>),
    Pin1: (pin1, 1, Input<Floating>),
    Pin2: (pin2, 2, Input<Floating>),
    Pin3: (pin3, 3, Input<Floating>),
    Pin4: (pin4, 4, Input<Floating>),
    Pin5: (pin5, 5, Input<Floating>),
    Pin6: (pin6, 6, Input<Floating>),
    Pin7: (pin7, 7, Input<Floating>),
    Pin8: (pin8, 8, Input<Floating>),
    Pin9: (pin9, 9, Input<Floating>),
    Pin10: (pin10, 10, Input<Floating>),
    Pin11: (pin11, 11, Input<Floating>),
    Pin12: (pin12, 12, Input<Floating>),
    Pin13: (pin13, 13, Input<Floating>),
    Pin14: (pin14, 14, Input<Floating>),
    Pin15: (pin15, 15, Input<Floating>),
    Pin16: (pin16, 16, Input<Floating>),
    Pin17: (pin17, 17, Input<Floating>),
    Pin18: (pin18, 18, Input<Floating>),
    Pin19: (pin19, 19, Input<Floating>),
    Pin20: (pin20, 20, Input<Floating>),
    Pin21: (pin21, 21, Input<Floating>),
    Pin22: (pin22, 22, Input<Floating>),
    Pin23: (pin23, 23, Input<Floating>),
    Pin24: (pin24, 24, Input<Floating>),
    Pin25: (pin25, 25, Input<Floating>),
    Pin26: (pin26, 26, Input<Floating>),
    Pin27: (pin27, 27, Input<Floating>),
    Pin28: (pin28, 28, Input<Floating>),
    Pin29: (pin29, 29, Input<Floating>),
    Pin30: (pin30, 30, Input<Floating>),
    Pin31: (pin31, 31, Input<Floating>),
]);
