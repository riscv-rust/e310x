//! # Digital I/O
//! # Note
//!
//! Implementation of the Async Embedded HAL I/O functionality.
//!

macro_rules! gpio_async {
    ($GPIOX:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $handle:ident),)+
    ]) => {
        use crate::asynch::poll_fn;
        use crate::gpio::*;
        use crate::gpio::gpio0::*;
        use embedded_hal::digital::{InputPin, Error, ErrorType, ErrorKind};
        use embedded_hal_async::digital::Wait;
        use e310x::$GPIOX;
        use core::task::{Poll, Waker};
        use core::cell::RefCell;
        use critical_section::Mutex;

        /// Error type for wait trait.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DigitalError {
            /// Error indicating that a wait operation was already in progress.
            AlreadyWaiting,
            /// Other errors.
            Other,
        }

        struct PinWaker {
            waker: Waker,
        }

        const N_PINS: usize = 32;
        static PIN_WAKERS: Mutex<RefCell<[Option<PinWaker>; N_PINS]>> =
        Mutex::new(RefCell::new([const{None}; N_PINS]));

        impl Error for DigitalError {
            fn kind(&self) -> ErrorKind {
                ErrorKind::Other
            }
        }

        impl PinWaker {
            fn new(waker: Waker) -> Self {
                Self { waker }
            }
            /// Consumes the timer and wakes its associated waker.
            #[inline]
            pub fn wake(self) {
                self.waker.wake();
            }
        }

        /// Interrupt handler for GPIO pins.
        fn on_irq(pin_n: usize) {
            let gpio_block = unsafe { Gpio0::steal() };

            let high_ip = gpio_block.high_ip().read().bits();
            let low_ip = gpio_block.low_ip().read().bits();
            let rise_ip = gpio_block.rise_ip().read().bits();
            let fall_ip = gpio_block.fall_ip().read().bits();
            let ip = high_ip | low_ip | rise_ip | fall_ip;
            let pin_mask = 1 << pin_n;
            let pin_pending = ip & pin_mask;

            // Disable the interrupt for the pin
            unsafe{
                gpio_block.high_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
                gpio_block.low_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
                gpio_block.rise_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
                gpio_block.fall_ie().modify(|r, w| w.bits(r.bits() &! pin_mask));
            }

            // Wake the pin if its interrupt is pending
            if pin_pending != 0 {
                critical_section::with(|cs| {
                    let mut pin_wakers = PIN_WAKERS.borrow_ref_mut(cs);
                        if let Some(pinwaker) = pin_wakers[pin_n].take() {
                            pinwaker.wake();
                        }
                });
            }

            // Clear pending pin interrupts
            unsafe{
                gpio_block.high_ip().modify(|r, w| w.bits(r.bits() | pin_mask));
                gpio_block.low_ip().modify(|r, w| w.bits(r.bits() | pin_mask));
                gpio_block.rise_ip().modify(|r, w| w.bits(r.bits() | pin_mask));
                gpio_block.fall_ip().modify(|r, w| w.bits(r.bits() | pin_mask));
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

                    // Enable the high interrupt for the pin.
                    let gpio_block = unsafe { $GPIOX::steal() };
                    gpio_block.high_ie().write(|w| w.$pxi().set_bit());

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        let result = critical_section::with(|cs| {
                            let pinwaker = &mut PIN_WAKERS.borrow_ref_mut(cs)[$i];
                            if gpio_block.high_ie().read().$pxi().bit_is_clear() {
                                Poll::Ready(Ok(()))
                            } else {
                                *pinwaker = Some(PinWaker::new(cx.waker().clone()));
                                Poll::Pending
                            }
                        });
                        result
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

                    // Enable the low interrupt for the pin.
                    let gpio_block = unsafe { $GPIOX::steal() };
                    gpio_block.low_ie().write(|w| w.$pxi().set_bit());

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        let result = critical_section::with(|cs| {
                            let pinwaker = &mut PIN_WAKERS.borrow_ref_mut(cs)[$i];
                            if gpio_block.low_ie().read().$pxi().bit_is_clear() {
                                Poll::Ready(Ok(()))
                            } else {
                                *pinwaker = Some(PinWaker::new(cx.waker().clone()));
                                Poll::Pending
                            }
                        });
                        result
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

                    // Enable the rising edge interrupt for the pin.
                    let gpio_block = unsafe { $GPIOX::steal() };
                    gpio_block.rise_ie().write(|w| w.$pxi().set_bit());

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        let result = critical_section::with(|cs| {
                            let pinwaker = &mut PIN_WAKERS.borrow_ref_mut(cs)[$i];
                            if gpio_block.rise_ie().read().$pxi().bit_is_clear() {
                                Poll::Ready(Ok(()))
                            } else {
                                *pinwaker = Some(PinWaker::new(cx.waker().clone()));
                                Poll::Pending
                            }
                        });
                        result
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

                    // Enable the falling edge interrupt for the pin.
                    let gpio_block = unsafe { $GPIOX::steal() };
                    gpio_block.fall_ie().write(|w| w.$pxi().set_bit());

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        let result = critical_section::with(|cs| {
                            let pinwaker = &mut PIN_WAKERS.borrow_ref_mut(cs)[$i];
                            if gpio_block.fall_ie().read().$pxi().bit_is_clear() {
                                Poll::Ready(Ok(()))
                            } else {
                                *pinwaker = Some(PinWaker::new(cx.waker().clone()));
                                Poll::Pending
                            }
                        });
                        result
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

                    // Enable the rising and falling edge interrupts for the pin.
                    let gpio_block = unsafe { $GPIOX::steal() };
                    gpio_block.rise_ie().write(|w| w.$pxi().set_bit());
                    gpio_block.fall_ie().write(|w| w.$pxi().set_bit());

                    // Await until an interrupt indicates that the pin has transitioned high.
                    poll_fn(|cx| {
                        let result = critical_section::with(|cs| {
                            if (gpio_block.fall_ie().read().$pxi().bit_is_clear()  ||
                                gpio_block.rise_ie().read().$pxi().bit_is_clear()) {
                                Poll::Ready(Ok(()))
                            } else {
                                let pinwaker = &mut PIN_WAKERS.borrow_ref_mut(cs)[$i];
                                *pinwaker = Some(PinWaker::new(cx.waker().clone()));
                                Poll::Pending
                            }
                        });
                        result
                    }).await
                }
            }

            /// Pin Interrupt Handler
            #[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::$handle)]
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
