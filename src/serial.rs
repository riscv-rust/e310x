//! Serial interface
//!
//! You can use the `Serial` interface with these UART instances
//!
//! # UART0
//! - TX: Pin 17 IOF0
//! - RX: Pin 16 IOF0
//! - Interrupt::UART0
//!
//! # UART1
//! - TX: Pin 25 IOF0
//! - RX: Pin 24 IOF0
//! - Interrupt::UART1
//! Pins not connected to package in E310-G000

use core::marker::PhantomData;

use hal::serial;
use nb;
use void::Void;

use e310x::UART0;
use clock::Clocks;
use gpio::{IOF0, gpio0};
use time::Bps;

// FIXME these should be "closed" traits
/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

unsafe impl<T> TxPin<UART0> for gpio0::Pin17<IOF0<T>> {}
unsafe impl<T> RxPin<UART0> for gpio0::Pin16<IOF0<T>> {}

/// Serial abstraction
pub struct Serial<USART, PINS> {
    uart: USART,
    pins: PINS,
}

/// Serial receiver
pub struct Rx<UART> {
    _usart: PhantomData<UART>,
}

/// Serial transmitter
pub struct Tx<UART> {
    _usart: PhantomData<UART>,
}

macro_rules! hal {
    ($(
        $UARTX:ident: $uartX:ident
    )+) => {
        $(
            impl<TX, RX> Serial<$UARTX, (TX, RX)> {
                /// Configures a UART peripheral to provide serial communication
                pub fn $uartX(
                    uart: $UARTX,
                    pins: (TX, RX),
                    baud_rate: Bps,
                    clocks: Clocks,
                ) -> Self
                where
                    TX: TxPin<$UARTX>,
                    RX: RxPin<$UARTX>,
                {
                    let div = clocks.coreclk().0 / baud_rate.0 + 1;
                    unsafe { uart.div.write(|w| w.bits(div)); }

                    uart.txctrl.write(|w| w.enable().bit(true));
                    uart.rxctrl.write(|w| w.enable().bit(true));

                    Serial { uart, pins }
                }

                /// Starts listening for an interrupt event
                pub fn listen(self) -> Self {
                    self.uart.ie.write(|w| w.txwm().bit(false)
                                       .rxwm().bit(true));
                    self
                }

                /// Starts listening for an interrupt event
                pub fn unlisten(self) -> Self {
                    self.uart.ie.write(|w| w.txwm().bit(false)
                                       .rxwm().bit(false));
                    self
                }

                /// Splits the `Serial` abstraction into a transmitter and a
                /// receiver half
                pub fn split(self) -> (Tx<$UARTX>, Rx<$UARTX>) {
                    (
                        Tx {
                            _usart: PhantomData,
                        },
                        Rx {
                            _usart: PhantomData,
                        },
                    )
                }

                /// Releases the USART peripheral and associated pins
                pub fn free(self) -> ($UARTX, (TX, RX)) {
                    (self.uart, self.pins)
                }
            }

            impl serial::Read<u8> for Rx<$UARTX> {
                type Error = Void;

                fn read(&mut self) -> nb::Result<u8, Void> {
                    // NOTE(unsafe) atomic read with no side effects
                    let rxdata = unsafe { (*$UARTX::ptr()).rxdata.read() };

                    if rxdata.empty().bit_is_set() {
                        Err(::nb::Error::WouldBlock)
                    } else {
                        Ok(rxdata.data().bits() as u8)
                    }
                }
            }

            impl serial::Write<u8> for Tx<$UARTX> {
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    // NOTE(unsafe) atomic read with no side effects
                    let txdata = unsafe { (*$UARTX::ptr()).txdata.read() };

                    if txdata.full().bit_is_set() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        Ok(())
                    }
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    // NOTE(unsafe) atomic read with no side effects
                    let txdata = unsafe { (*$UARTX::ptr()).txdata.read() };

                    if txdata.full().bit_is_set() {
                        Err(::nb::Error::WouldBlock)
                    } else {
                        unsafe {
                            (*$UARTX::ptr()).txdata
                                .write(|w| w.data().bits(byte));
                        }
                        Ok(())
                    }
                }
            }
        )+
    }
}

hal! {
    UART0: uart0
}
