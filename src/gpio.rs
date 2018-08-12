//! General Purpose I/O

use core::marker::PhantomData;

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


macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, [
        $($PXi:ident: ($pxi:ident, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::{InputPin, OutputPin, StatefulOutputPin};
            use e310x::{$gpioy, $GPIOX};
            use super::{IOF0, IOF1, Drive, Floating, GpioExt, Input, Invert,
                        NoInvert, Output, PullUp, Regular};

            /// GPIO parts for fine grained permission control.
            pub struct Parts {
                /// Opaque INPUT_EN register
                pub input_en: INPUT_EN,
                /// Opaque OUTPUT_EN register
                pub output_en: OUTPUT_EN,
                /// Opaque PULLUP register
                pub pullup: PULLUP,
                /// Opaque DRIVE register
                pub drive: DRIVE,
                /// Opaque OUT_XOR register
                pub out_xor: OUT_XOR,
                /// Opaque IOF_EN register
                pub iof_en: IOF_EN,
                /// Opaque IOF_SEL register
                pub iof_sel: IOF_SEL,
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        input_en: INPUT_EN { _0: () },
                        output_en: OUTPUT_EN { _0: () },
                        pullup: PULLUP { _0: () },
                        drive: DRIVE { _0: () },
                        out_xor: OUT_XOR { _0: () },
                        iof_en: IOF_EN { _0: () },
                        iof_sel: IOF_SEL { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Opaque INPUT_EN register
            #[allow(non_camel_case_types)]
            pub struct INPUT_EN {
                _0: (),
            }

            impl INPUT_EN {
                pub(crate) fn input_en(&mut self) -> &$gpioy::INPUT_EN {
                    unsafe { &(*$GPIOX::ptr()).input_en }
                }
            }

            /// Opaque OUTPUT_EN register
            #[allow(non_camel_case_types)]
            pub struct OUTPUT_EN {
                _0: (),
            }

            impl OUTPUT_EN {
                pub(crate) fn output_en(&mut self) -> &$gpioy::OUTPUT_EN {
                    unsafe { &(*$GPIOX::ptr()).output_en }
                }
            }

            /// Opaque PULLUP register
            pub struct PULLUP {
                _0: (),
            }

            impl PULLUP {
                pub(crate) fn pullup(&mut self) -> &$gpioy::PULLUP {
                    unsafe { &(*$GPIOX::ptr()).pullup }
                }
            }

            /// Opaque DRIVE register
            pub struct DRIVE {
                _0: (),
            }

            impl DRIVE {
                pub(crate) fn drive(&mut self) -> &$gpioy::DRIVE {
                    unsafe { &(*$GPIOX::ptr()).drive }
                }
            }


            /// Opaque OUT_XOR register
            #[allow(non_camel_case_types)]
            pub struct OUT_XOR {
                _0: (),
            }

            impl OUT_XOR {
                pub(crate) fn out_xor(&mut self) -> &$gpioy::OUT_XOR {
                    unsafe { &(*$GPIOX::ptr()).out_xor }
                }
            }

            /// Opaque IOF_EN register
            #[allow(non_camel_case_types)]
            pub struct IOF_EN {
                _0: (),
            }

            impl IOF_EN {
                pub(crate) fn iof_en(&mut self) -> &$gpioy::IOF_EN {
                    unsafe { &(*$GPIOX::ptr()).iof_en }
                }
            }

            /// Opaque IOF_SEL register
            #[allow(non_camel_case_types)]
            pub struct IOF_SEL {
                _0: (),
            }

            impl IOF_SEL {
                pub(crate) fn iof_sel(&mut self) -> &$gpioy::IOF_SEL {
                    unsafe { &(*$GPIOX::ptr()).iof_sel }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to serve as alternate function 0 (AF0)
                    pub fn into_iof0(
                        self,
                        out_xor: &mut OUT_XOR,
                        iof_sel: &mut IOF_SEL,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<IOF0<NoInvert>> {
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(false));
                        iof_sel.iof_sel().modify(|_, w| w.$pxi().bit(false));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(true));

                        $PXi { _mode: PhantomData }
                    }


                    /// Configures the pin to serve as alternate function 1 (AF1)
                    pub fn into_iof1(
                        self,
                        out_xor: &mut OUT_XOR,
                        iof_sel: &mut IOF_SEL,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<IOF1<NoInvert>> {
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(false));
                        iof_sel.iof_sel().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(true));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as inverted alternate function 0 (AF0)
                    pub fn into_inverted_iof0(
                        self,
                        out_xor: &mut OUT_XOR,
                        iof_sel: &mut IOF_SEL,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<IOF0<Invert>> {
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(true));
                        iof_sel.iof_sel().modify(|_, w| w.$pxi().bit(false));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(true));

                        $PXi { _mode: PhantomData }
                    }


                    /// Configures the pin to serve as inverted alternate function 1 (AF1)
                    pub fn into_inverted_iof1(
                        self,
                        out_xor: &mut OUT_XOR,
                        iof_sel: &mut IOF_SEL,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<IOF1<Invert>> {
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(true));
                        iof_sel.iof_sel().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(true));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to serve as a floating input pin
                    pub fn into_floating_input(
                        self,
                        pullup: &mut PULLUP,
                        input_en: &mut INPUT_EN,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<Input<Floating>> {
                        pullup.pullup().modify(|_, w| w.$pxi().bit(false));
                        input_en.input_en().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(false));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_up_input(
                        self,
                        pullup: &mut PULLUP,
                        input_en: &mut INPUT_EN,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<Input<PullUp>> {
                        pullup.pullup().modify(|_, w| w.$pxi().bit(true));
                        input_en.input_en().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(false));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an output pin
                    pub fn into_output(
                        self,
                        output_en: &mut OUTPUT_EN,
                        drive: &mut DRIVE,
                        out_xor: &mut OUT_XOR,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<Output<Regular<NoInvert>>> {
                        drive.drive().modify(|_, w| w.$pxi().bit(false));
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(false));
                        output_en.output_en().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(false));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an inverted output pin
                    pub fn into_inverted_output(
                        self,
                        output_en: &mut OUTPUT_EN,
                        drive: &mut DRIVE,
                        out_xor: &mut OUT_XOR,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<Output<Regular<Invert>>> {
                        drive.drive().modify(|_, w| w.$pxi().bit(false));
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(true));
                        output_en.output_en().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(false));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configure the pin to operate as an output pin with high
                    /// current drive
                    pub fn into_output_drive(
                        self,
                        output_en: &mut OUTPUT_EN,
                        drive: &mut DRIVE,
                        out_xor: &mut OUT_XOR,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<Output<Drive<NoInvert>>> {
                        drive.drive().modify(|_, w| w.$pxi().bit(true));
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(false));
                        output_en.output_en().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(false));

                        $PXi { _mode: PhantomData }
                    }

                    /// Configure the pin to operate as an inverted output pin with
                    /// high current drive
                    pub fn into_inverted_output_drive(
                        self,
                        output_en: &mut OUTPUT_EN,
                        drive: &mut DRIVE,
                        out_xor: &mut OUT_XOR,
                        iof_en: &mut IOF_EN
                    ) -> $PXi<Output<Drive<Invert>>> {
                        drive.drive().modify(|_, w| w.$pxi().bit(true));
                        out_xor.out_xor().modify(|_, w| w.$pxi().bit(true));
                        output_en.output_en().modify(|_, w| w.$pxi().bit(true));
                        iof_en.iof_en().modify(|_, w| w.$pxi().bit(false));

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        let gpio = unsafe { &*$GPIOX::ptr() };
                        // NOTE unsafe atomic read with no side effects
                        gpio.value.read().$pxi().bit()
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    fn is_set_high(&self) -> bool {
                        !self.is_set_low()
                    }

                    fn is_set_low(&self) -> bool {
                        // NOTE unsafe atomic read with no side effects
                        let gpio = unsafe { &*$GPIOX::ptr() };
                        gpio.value.read().$pxi().bit()
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        // FIXME has to read register first
                        // use atomics
                        let gpio = unsafe { &*$GPIOX::ptr() };
                        gpio.port.modify(|_, w| w.$pxi().bit(true));
                    }

                    fn set_low(&mut self) {
                        // FIXME: has to read register first
                        // use atomics
                        let gpio = unsafe { &*$GPIOX::ptr() };
                        gpio.port.modify(|_, w| w.$pxi().bit(false));
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Toggles the pin state.
                    pub fn toggle(&mut self) {
                        // FIXME: has to read register first
                        // use atomics
                        let gpio = unsafe { &*$GPIOX::ptr() };
                        gpio.port.modify(|r, w| w.$pxi().bit(!r.$pxi().bit()));
                    }
                }
            )+
        }
    }
}

gpio!(GPIO0, gpio0, gpio0, [
    Pin0: (pin0, Input<Floating>),
    Pin1: (pin1, Input<Floating>),
    Pin2: (pin2, Input<Floating>),
    Pin3: (pin3, Input<Floating>),
    Pin4: (pin4, Input<Floating>),
    Pin5: (pin5, Input<Floating>),
    Pin6: (pin6, Input<Floating>),
    Pin7: (pin7, Input<Floating>),
    Pin8: (pin8, Input<Floating>),
    Pin9: (pin9, Input<Floating>),
    Pin10: (pin10, Input<Floating>),
    Pin11: (pin11, Input<Floating>),
    Pin12: (pin12, Input<Floating>),
    Pin13: (pin13, Input<Floating>),
    Pin14: (pin14, Input<Floating>),
    Pin15: (pin15, Input<Floating>),
    Pin16: (pin16, Input<Floating>),
    Pin17: (pin17, Input<Floating>),
    Pin18: (pin18, Input<Floating>),
    Pin19: (pin19, Input<Floating>),
    Pin20: (pin20, Input<Floating>),
    Pin21: (pin21, Input<Floating>),
    Pin22: (pin22, Input<Floating>),
    Pin23: (pin23, Input<Floating>),
    Pin24: (pin24, Input<Floating>),
    Pin25: (pin25, Input<Floating>),
    Pin26: (pin26, Input<Floating>),
    Pin27: (pin27, Input<Floating>),
    Pin28: (pin28, Input<Floating>),
    Pin29: (pin29, Input<Floating>),
    Pin30: (pin30, Input<Floating>),
    Pin31: (pin31, Input<Floating>),
]);
