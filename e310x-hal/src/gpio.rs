//! General Purpose I/O

use core::marker::PhantomData;

use portable_atomic::{AtomicU32, Ordering};

/// Event Type for GPIO interrupts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// High level event
    High,
    /// Low level event
    Low,
    /// Rising edge event
    Rise,
    /// Falling edge event
    Fall,
    /// Both levels event
    ///
    /// # Note
    ///
    /// In the methods that check if an interrupt is enabled or pending,
    /// this event type works like an **any** operator between `High` and `Low` events.
    BothLevels,
    /// Both edges event
    ///
    /// # Note
    ///
    /// In the methods that check if an interrupt is enabled or pending,
    /// this event type works like an **any** operator between `Rise` and `Fall` events.
    BothEdges,
    /// All events
    ///
    /// # Note
    ///
    /// In the methods that check if an interrupt is enabled or pending,
    /// this event type works like an **any** operator between all event types.
    All,
}

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
    fn peripheral() -> e310x::Gpio0;

    fn input_value(index: usize) -> bool {
        let p = Self::peripheral();
        (p.input_val().read().bits() >> (index & 31) & 1) != 0
    }

    fn set_input_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.input_en()) };
        atomic_set_bit(r, index, bit);
    }

    fn set_output_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.output_en()) };
        atomic_set_bit(r, index, bit);
    }

    fn output_value(index: usize) -> bool {
        let p = Self::peripheral();
        ((p.output_val().read().bits() >> (index & 31)) & 1) != 0
    }

    fn set_output_value(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.output_val()) };
        atomic_set_bit(r, index, bit);
    }

    fn toggle_pin(index: usize) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.output_val()) };
        let mask = 1 << (index & 31);
        r.fetch_xor(mask, Ordering::SeqCst);
    }

    fn set_pullup(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.pullup()) };
        atomic_set_bit(r, index, bit);
    }

    fn set_drive(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.drive()) };
        atomic_set_bit(r, index, bit);
    }

    fn set_out_xor(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.out_xor()) };
        atomic_set_bit(r, index, bit);
    }

    fn set_iof_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.iof_en()) };
        atomic_set_bit(r, index, bit);
    }

    fn set_iof_sel(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { core::mem::transmute(p.iof_sel()) };
        atomic_set_bit(r, index, bit);
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $handle:ident, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;
            use core::convert::Infallible;

            use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ErrorType};
            use e310x::{$GPIOX, Plic, interrupt::{ExternalInterrupt, Priority}};
            use super::{Unknown, IOF0, IOF1, Drive, Floating, GpioExt, Input, Invert,
                        NoInvert, Output, PullUp, Regular, PinIndex, PeripheralAccess, EventType};

            /// GPIO parts for fine grained permission control.
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl PeripheralAccess for $GPIOX {
                #[inline(always)]
                fn peripheral() -> e310x::Gpio0 {
                    unsafe { $GPIOX::steal() }
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

                    /// Enables the external interrupt source for the pin.
                    ///
                    /// # Note
                    ///
                    /// This function enables the external interrupt source in the PLIC,
                    /// but does not enable the PLIC peripheral itself. For more details,
                    /// refer to the [`e310x::Plic`] documentation.
                    ///
                    /// # Safety
                    ///
                    /// Enabling an interrupt source can break mask-based critical sections.
                    pub unsafe fn enable_exti(&mut self, plic: &Plic) {
                        let ctx = plic.ctx0();
                        ctx.enables().enable(ExternalInterrupt::$handle);
                    }

                    /// Disables the external interrupt source for the pin.
                    pub fn disable_exti(&mut self, plic: &Plic) {
                        let ctx = plic.ctx0();
                        ctx.enables().disable(ExternalInterrupt::$handle);
                    }

                    /// Returns whether the external interrupt source for the pin is enabled.
                    pub fn is_exti_enabled(&self, plic: &Plic) -> bool {
                        let ctx = plic.ctx0();
                        ctx.enables().is_enabled(ExternalInterrupt::$handle)
                    }

                    /// Sets the external interrupt source priority.
                    ///
                    ///  # Safety
                    ///
                    ///  Changing the priority level can break priority-based critical sections.
                    pub unsafe fn set_exti_priority(&mut self, plic: &Plic, priority: Priority) {
                        let priorities = plic.priorities();
                        priorities.set_priority(ExternalInterrupt::$handle, priority);
                    }

                    /// Returns the external interrupt source priority.
                    pub fn get_exti_priority(&self, plic: &Plic) -> Priority {
                        let priorities = plic.priorities();
                        priorities.get_priority(ExternalInterrupt::$handle)
                    }

                    /// Enables the selected interrupts for the pin in the interrupt enable registers
                    ///
                    /// # Note
                    ///
                    ///  This function does not enable the interrupt in the PLIC, it only sets the
                    /// interrupt enable bit in the GPIO peripheral. You must call
                    /// [`enable_exti()`](Self::enable_exti) to enable the interrupt in
                    /// the PLIC.
                    pub fn enable_interrupt(&mut self, event: EventType) {
                        let gpio_block = $GPIOX::peripheral();
                        let pin_mask = 1 << $i;

                        match event {
                            EventType::High => {
                                unsafe { gpio_block.high_ie().modify(|r, w| w.bits(r.bits() | pin_mask)); }
                            }
                            EventType::Low => {
                                unsafe { gpio_block.low_ie().modify(|r, w| w.bits(r.bits() | pin_mask)); }
                            }
                            EventType::BothLevels => {
                                unsafe {
                                    gpio_block.high_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                    gpio_block.low_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                }
                            }
                            EventType::Rise => {
                                unsafe { gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() | pin_mask)); }
                            }
                            EventType::Fall => {
                                unsafe { gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() | pin_mask)); }
                            }
                            EventType::BothEdges => {
                                unsafe {
                                    gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                    gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                }
                            }
                            EventType::All => {
                                unsafe {
                                    gpio_block.high_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                    gpio_block.low_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                    gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                    gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() | pin_mask));
                                }
                            }
                        }
                    }

                    /// Disables the selected interrupts for the pin in the interrupt enable registers
                    pub fn disable_interrupt(&mut self, event: EventType) {
                        let gpio_block = $GPIOX::peripheral();
                        let pin_mask = 1 << $i;

                        match event {
                            EventType::High => {
                                unsafe { gpio_block.high_ie().modify(|r, w| w.bits(r.bits() & !pin_mask)); }
                            }
                            EventType::Low => {
                                unsafe { gpio_block.low_ie().modify(|r, w| w.bits(r.bits() & !pin_mask)); }
                            }
                            EventType::BothLevels => {
                                unsafe {
                                    gpio_block.high_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                    gpio_block.low_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                }
                            }
                            EventType::Rise => {
                                unsafe { gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() & !pin_mask)); }
                            }
                            EventType::Fall => {
                                unsafe { gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() & !pin_mask)); }
                            }
                            EventType::BothEdges => {
                                unsafe {
                                    gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                    gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                }
                            }
                            EventType::All => {
                                unsafe {
                                    gpio_block.high_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                    gpio_block.low_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                    gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                    gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() & !pin_mask));
                                }
                            }
                        }
                    }

                    /// Clears pending interrupts for the selected pin interrupts.
                    pub fn clear_interrupt(&mut self, event: EventType) {
                        let gpio_block = $GPIOX::peripheral();
                        let pin_mask = 1 << $i;

                        match event {
                            EventType::High => {
                                unsafe { gpio_block.high_ip().write(|w| w.bits(pin_mask)); }
                            }
                            EventType::Low => {
                                unsafe { gpio_block.low_ip().write(|w| w.bits(pin_mask)); }
                            }
                            EventType::BothLevels => {
                                unsafe {
                                    gpio_block.high_ip().write(|w| w.bits(pin_mask));
                                    gpio_block.low_ip().write(|w| w.bits(pin_mask));
                                }
                            }
                            EventType::Rise => {
                                unsafe { gpio_block.rise_ip().write(|w| w.bits(pin_mask)); }
                            }
                            EventType::Fall => {
                                unsafe { gpio_block.fall_ip().write(|w| w.bits(pin_mask)); }
                            }
                            EventType::BothEdges => {
                                unsafe {
                                    gpio_block.rise_ip().write(|w| w.bits(pin_mask));
                                    gpio_block.fall_ip().write(|w| w.bits(pin_mask));
                                }
                            }
                            EventType::All => {
                                unsafe {
                                    gpio_block.high_ip().write(|w| w.bits(pin_mask));
                                    gpio_block.low_ip().write(|w| w.bits(pin_mask));
                                    gpio_block.rise_ip().write(|w| w.bits(pin_mask));
                                    gpio_block.fall_ip().write(|w| w.bits(pin_mask));
                                }
                            }
                        }
                    }

                    /// Returns true if the interrupt for the pin is enabled.
                    ///
                    ///  # Note
                    ///
                    ///  [EventType::BothEdges] will return true if either the
                    ///  rising or falling edge interrupts are enabled,
                    ///  [EventType::BothLevels] will return true if either the
                    ///  high or low level interrupts are enabled
                    ///  and [EventType::All] will return true if any of the
                    ///  interrupts are enabled.
                    pub fn is_interrupt_enabled(&self, event: EventType) -> bool {
                        let gpio_block = $GPIOX::peripheral();
                        let pin_mask = 1 << $i;

                        match event {
                            EventType::High => gpio_block.high_ie().read().bits() & pin_mask != 0,
                            EventType::Low => gpio_block.low_ie().read().bits() & pin_mask != 0,
                            EventType::BothLevels => {
                                (gpio_block.high_ie().read().bits() & pin_mask != 0) ||
                                (gpio_block.low_ie().read().bits() & pin_mask != 0)
                            }
                            EventType::Rise => gpio_block.rise_ie().read().bits() & pin_mask != 0,
                            EventType::Fall => gpio_block.fall_ie().read().bits() & pin_mask != 0,
                            EventType::BothEdges => {
                                (gpio_block.rise_ie().read().bits() & pin_mask != 0) ||
                                (gpio_block.fall_ie().read().bits() & pin_mask != 0)
                            }
                            EventType::All => {
                                (gpio_block.high_ie().read().bits() & pin_mask != 0) ||
                                (gpio_block.low_ie().read().bits() & pin_mask != 0) ||
                                (gpio_block.rise_ie().read().bits() & pin_mask != 0) ||
                                (gpio_block.fall_ie().read().bits() & pin_mask != 0)
                            }
                        }
                    }

                    /// Returns true if the interrupt for the pin is pending.
                    ///
                    ///  # Note
                    ///
                    ///  [EventType::BothEdges] will return true if either the
                    ///  rising or falling edge interrupts are pending,
                    ///  [EventType::BothLevels] will return true if either the
                    ///  high or low level interrupts are pending
                    ///  and [EventType::All] will return true if any of the
                    ///  interrupts are pending.
                    pub fn is_interrupt_pending(&self, event: EventType) -> bool {
                        let gpio_block = $GPIOX::peripheral();
                        let pin_mask = 1 << $i;

                        match event {
                            EventType::High => gpio_block.high_ip().read().bits() & pin_mask != 0,
                            EventType::Low => gpio_block.low_ip().read().bits() & pin_mask != 0,
                            EventType::BothLevels => {
                                (gpio_block.high_ip().read().bits() & pin_mask != 0) ||
                                (gpio_block.low_ip().read().bits() & pin_mask != 0)
                            }
                            EventType::Rise => gpio_block.rise_ip().read().bits() & pin_mask != 0,
                            EventType::Fall => gpio_block.fall_ip().read().bits() & pin_mask != 0,
                            EventType::BothEdges => {
                                (gpio_block.rise_ip().read().bits() & pin_mask != 0) ||
                                (gpio_block.fall_ip().read().bits() & pin_mask != 0)
                            }
                            EventType::All => {
                                (gpio_block.high_ip().read().bits() & pin_mask != 0) ||
                                (gpio_block.low_ip().read().bits() & pin_mask != 0) ||
                                (gpio_block.rise_ip().read().bits() & pin_mask != 0) ||
                                (gpio_block.fall_ip().read().bits() & pin_mask != 0)
                            }
                        }
                    }
                }

                impl<MODE> ErrorType for $PXi<Input<MODE>> {
                    type Error = Infallible;
                }

                impl<MODE> ErrorType for $PXi<Output<MODE>> {
                    type Error = Infallible;
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    #[inline]
                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        Ok($GPIOX::input_value(Self::INDEX))
                    }

                    #[inline]
                    fn is_low(&mut self) -> Result<bool, Self::Error> {
                        Ok(!self.is_high()?)
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    #[inline]
                    fn set_high(&mut self) -> Result<(), Infallible> {
                        $GPIOX::set_output_value(Self::INDEX, true);
                        Ok(())
                    }

                    #[inline]
                    fn set_low(&mut self) -> Result<(), Infallible> {
                        $GPIOX::set_output_value(Self::INDEX, false);
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    #[inline]
                    fn is_set_high(&mut self) -> Result<bool, Infallible> {
                        Ok($GPIOX::output_value(Self::INDEX))
                    }

                    #[inline]
                    fn is_set_low(&mut self) -> Result<bool, Infallible> {
                        Ok(!self.is_set_high()?)
                    }

                    #[inline]
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
gpio!(Gpio0, gpio0, [
    Pin0: (pin0, 0, GPIO0, Unknown),
    Pin1: (pin1, 1, GPIO1, Unknown),
    Pin2: (pin2, 2, GPIO2, Unknown),
    Pin3: (pin3, 3, GPIO3, Unknown),
    Pin4: (pin4, 4, GPIO4, Unknown),
    Pin5: (pin5, 5, GPIO5, Unknown),
    Pin6: (pin6, 6, GPIO6, Unknown),
    Pin7: (pin7, 7, GPIO7, Unknown),
    Pin8: (pin8, 8, GPIO8, Unknown),
    Pin9: (pin9, 9, GPIO9, Unknown),
    Pin10: (pin10, 10, GPIO10, Unknown),
    Pin11: (pin11, 11, GPIO11, Unknown),
    Pin12: (pin12, 12, GPIO12, Unknown),
    Pin13: (pin13, 13, GPIO13, Unknown),
    Pin14: (pin14, 14, GPIO14, Unknown),
    Pin15: (pin15, 15, GPIO15, Unknown),
    Pin16: (pin16, 16, GPIO16, Unknown),
    Pin17: (pin17, 17, GPIO17, Unknown),
    Pin18: (pin18, 18, GPIO18, Unknown),
    Pin19: (pin19, 19, GPIO19, Unknown),
    Pin20: (pin20, 20, GPIO20, Unknown),
    Pin21: (pin21, 21, GPIO21, Unknown),
    Pin22: (pin22, 22, GPIO22, Unknown),
    Pin23: (pin23, 23, GPIO23, Unknown),
    Pin24: (pin24, 24, GPIO24, Unknown),
    Pin25: (pin25, 25, GPIO25, Unknown),
    Pin26: (pin26, 26, GPIO26, Unknown),
    Pin27: (pin27, 27, GPIO27, Unknown),
    Pin28: (pin28, 28, GPIO28, Unknown),
    Pin29: (pin29, 29, GPIO29, Unknown),
    Pin30: (pin30, 30, GPIO30, Unknown),
    Pin31: (pin31, 31, GPIO31, Unknown),
]);
