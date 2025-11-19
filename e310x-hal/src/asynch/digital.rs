//! # Digital I/O
//! # Note
//!
//! Implementation of the Async Embedded HAL I/O functionality.
//!

macro_rules! gpio_async {
    ($GPIOX:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $handle:ident),)+
    ]) => {
        use core::cell::RefCell;
        use core::task::{Poll, Waker};
        use core::future::poll_fn;
        use critical_section::Mutex;
        use crate::gpio::*;
        use crate::gpio::gpio0::*;
        use e310x::$GPIOX;
        use e310x::interrupt::ExternalInterrupt;
        use embedded_hal::digital::{Error, ErrorKind, ErrorType, InputPin};
        use embedded_hal_async::digital::Wait;

        /// Error type for wait trait.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DigitalError {
            /// Error indicating that a wait operation was already in progress.
            AlreadyWaiting,
            /// Other errors.
            Other,
        }

        const N_PINS: usize = 32;
        static PIN_WAKERS: Mutex<RefCell<[Option<Waker>; N_PINS]>> =
        Mutex::new(RefCell::new([const{None}; N_PINS]));

        impl Error for DigitalError {
            fn kind(&self) -> ErrorKind {
                ErrorKind::Other
            }
        }

        /// Interrupt handler for GPIO pins.
        #[inline]
        fn on_irq(pin_n: usize) {
            let gpio_block = unsafe { $GPIOX::steal() };
            let pin_mask = 1 << pin_n;

            // Disable the interrupt for the pin
            unsafe{
                gpio_block.high_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
                gpio_block.low_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
                gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
                gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
            }

            // Wake the pin if possible
            critical_section::with(|cs| {
                let mut pin_wakers = PIN_WAKERS.borrow_ref_mut(cs);
                if let Some(pinwaker) = pin_wakers[pin_n].take() {
                    pinwaker.wake();
                }
            });

            // Clear pending pin interrupts
            unsafe{
                gpio_block.high_ip().write(|w| w.bits(pin_mask));
                gpio_block.low_ip().write(|w| w.bits(pin_mask));
                gpio_block.rise_ip().write(|w| w.bits(pin_mask));
                gpio_block.fall_ip().write(|w| w.bits(pin_mask));
            }
        }

        /// GPIO
        $(
            impl<MODE> ErrorType for $PXi<Input<MODE>> {
                type Error = DigitalError;
            }
            /// Wait trait implementation
            impl<MODE> Wait for $PXi<Input<MODE>> {
                #[inline]
                async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
                    // If the pin is already high, no need to wait.
                    if self.is_high().unwrap() {
                        return Ok(());
                    }

                    // Prevent concurrent waiters.
                    if critical_section::with(|cs| {
                        PIN_WAKERS.borrow_ref(cs)[$i].is_some()
                    }){
                        return Err(DigitalError::AlreadyWaiting);
                    }

                    // Clear previous high interrupts for the pin.
                    self.clear_interrupt(EventType::High);

                    // Enable the high interrupt for the pin.
                    self.enable_interrupt(EventType::High);

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        if !self.is_interrupt_enabled(EventType::High) {
                            Poll::Ready(Ok(()))
                        } else {
                            critical_section::with(|cs| {
                                let mut pinwaker = PIN_WAKERS.borrow_ref_mut(cs);
                                pinwaker[$i] = Some(cx.waker().clone());
                            });
                            Poll::Pending
                        }
                    }).await
                }

                #[inline]
                async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
                    // If the pin is already low, no need to wait.
                    if self.is_low().unwrap() {
                        return Ok(());
                    }

                    // Prevent concurrent waiters.
                    if critical_section::with(|cs| {
                        PIN_WAKERS.borrow_ref(cs)[$i].is_some()
                    }){
                        return Err(DigitalError::AlreadyWaiting);
                    }

                    // Clear previous low interrupts for the pin.
                    self.clear_interrupt(EventType::Low);

                    // Enable the low interrupt for the pin.
                    self.enable_interrupt(EventType::Low);

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        if !self.is_interrupt_enabled(EventType::Low) {
                            Poll::Ready(Ok(()))
                        } else {
                            critical_section::with(|cs| {
                                let mut pinwaker = PIN_WAKERS.borrow_ref_mut(cs);
                                pinwaker[$i] = Some(cx.waker().clone());
                            });
                            Poll::Pending
                        }
                    }).await
                }

                #[inline]
                async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
                    // Prevent concurrent waiters.
                    if critical_section::with(|cs| {
                        PIN_WAKERS.borrow_ref(cs)[$i].is_some()
                    }){
                        return Err(DigitalError::AlreadyWaiting);
                    }

                    // Clear previous rising edge interrupts for the pin.
                    self.clear_interrupt(EventType::Rise);

                    // Enable the rising edge interrupt for the pin.
                    self.enable_interrupt(EventType::Rise);

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        if !self.is_interrupt_enabled(EventType::Rise) {
                            Poll::Ready(Ok(()))
                        } else {
                            critical_section::with(|cs| {
                                let mut pinwaker = PIN_WAKERS.borrow_ref_mut(cs);
                                pinwaker[$i] = Some(cx.waker().clone());
                            });
                            Poll::Pending
                        }
                    }).await
                }

                #[inline]
                async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
                    // Prevent concurrent waiters.
                    if critical_section::with(|cs| {
                        PIN_WAKERS.borrow_ref(cs)[$i].is_some()
                    }){
                        return Err(DigitalError::AlreadyWaiting);
                    }

                    // Clear previous falling edge interrupts for the pin.
                    self.clear_interrupt(EventType::Fall);

                    // Enable the falling edge interrupt for the pin.
                    self.enable_interrupt(EventType::Fall);

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        if !self.is_interrupt_enabled(EventType::Fall) {
                            Poll::Ready(Ok(()))
                        } else {
                            critical_section::with(|cs| {
                                let mut pinwaker = PIN_WAKERS.borrow_ref_mut(cs);
                                pinwaker[$i] = Some(cx.waker().clone());
                            });
                            Poll::Pending
                        }
                    }).await
                }

                #[inline]
                async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
                    // Prevent concurrent waiters.
                    if critical_section::with(|cs| {
                        PIN_WAKERS.borrow_ref(cs)[$i].is_some()
                    }){
                        return Err(DigitalError::AlreadyWaiting);
                    }

                    // Clear previous rising and falling edge interrupts for the pin.
                    self.clear_interrupt(EventType::BothEdges);

                    // Enable the rising and falling edge interrupts for the pin.
                    self.enable_interrupt(EventType::BothEdges);

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        if !self.is_interrupt_enabled(EventType::BothEdges) {
                            Poll::Ready(Ok(()))
                        } else {
                            critical_section::with(|cs| {
                                let mut pinwaker = PIN_WAKERS.borrow_ref_mut(cs);
                                pinwaker[$i] = Some(cx.waker().clone());
                            });
                            Poll::Pending
                        }
                    }).await
                }
            }

            /// Pin Interrupt Handler
            #[riscv_rt::external_interrupt(ExternalInterrupt::$handle)]
            fn $pxi() {
                on_irq($i);
            }
        )+
    }
}

gpio_async!(Gpio0, [
    Pin0: (pin0, 0, GPIO0),
    Pin1: (pin1, 1, GPIO1),
    Pin2: (pin2, 2, GPIO2),
    Pin3: (pin3, 3, GPIO3),
    Pin4: (pin4, 4, GPIO4),
    Pin5: (pin5, 5, GPIO5),
    Pin6: (pin6, 6, GPIO6),
    Pin7: (pin7, 7, GPIO7),
    Pin8: (pin8, 8, GPIO8),
    Pin9: (pin9, 9, GPIO9),
    Pin10: (pin10, 10, GPIO10),
    Pin11: (pin11, 11, GPIO11),
    Pin12: (pin12, 12, GPIO12),
    Pin13: (pin13, 13, GPIO13),
    Pin14: (pin14, 14, GPIO14),
    Pin15: (pin15, 15, GPIO15),
    Pin16: (pin16, 16, GPIO16),
    Pin17: (pin17, 17, GPIO17),
    Pin18: (pin18, 18, GPIO18),
    Pin19: (pin19, 19, GPIO19),
    Pin20: (pin20, 20, GPIO20),
    Pin21: (pin21, 21, GPIO21),
    Pin22: (pin22, 22, GPIO22),
    Pin23: (pin23, 23, GPIO23),
    Pin24: (pin24, 24, GPIO24),
    Pin25: (pin25, 25, GPIO25),
    Pin26: (pin26, 26, GPIO26),
    Pin27: (pin27, 27, GPIO27),
    Pin28: (pin28, 28, GPIO28),
    Pin29: (pin29, 29, GPIO29),
    Pin30: (pin30, 30, GPIO30),
    Pin31: (pin31, 31, GPIO31),
]);
