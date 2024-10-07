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
//! *Warning:* UART1 pins are not connected to package in FE310-G000
//! - TX: Pin 18 IOF0
//! - RX: Pin 23 IOF0
//! - Interrupt::UART1

use core::convert::Infallible;
use core::ops::Deref;

use embedded_hal::serial;
use nb;

use crate::clock::Clocks;
use crate::gpio::{gpio0, IOF0};
use crate::time::Bps;
use core::mem;
#[allow(unused_imports)]
use e310x::{uart0, Uart0, Uart1};

// FIXME these should be "closed" traits
/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

unsafe impl<T> TxPin<Uart0> for gpio0::Pin17<IOF0<T>> {}
unsafe impl<T> RxPin<Uart0> for gpio0::Pin16<IOF0<T>> {}

#[cfg(feature = "g002")]
mod g002_ims {
    use super::{gpio0, RxPin, TxPin, Uart1, IOF0};
    unsafe impl<T> TxPin<Uart1> for gpio0::Pin18<IOF0<T>> {}
    unsafe impl<T> RxPin<Uart1> for gpio0::Pin23<IOF0<T>> {}
}

#[doc(hidden)]
pub trait UartX: Deref<Target = uart0::RegisterBlock> {}
impl UartX for Uart0 {}
impl UartX for Uart1 {}

/// Serial abstraction
pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

/// Serial receiver
pub struct Rx<UART> {
    uart: UART,
}

/// Serial transmitter
pub struct Tx<UART> {
    uart: UART,
}

impl<UART: UartX, TX, RX> Serial<UART, (TX, RX)> {
    /// Configures a UART peripheral to provide serial communication
    pub fn new(uart: UART, pins: (TX, RX), baud_rate: Bps, clocks: Clocks) -> Self
    where
        TX: TxPin<UART>,
        RX: RxPin<UART>,
    {
        let div = clocks.tlclk().0 / baud_rate.0 - 1;
        unsafe {
            uart.ie().write(|w| w.txwm().bit(false).rxwm().bit(false));
            uart.div().write(|w| w.bits(div));
            uart.txctrl()
                .write(|w| w.counter().bits(1).enable().bit(true));
            uart.rxctrl().write(|w| w.enable().bit(true));
        }

        Serial { uart, pins }
    }

    /// Starts listening for an interrupt event
    pub fn listen(self) -> Self {
        self.uart
            .ie()
            .write(|w| w.txwm().bit(false).rxwm().bit(true));
        self
    }

    /// Stops listening for an interrupt event
    pub fn unlisten(self) -> Self {
        self.uart
            .ie()
            .write(|w| w.txwm().bit(false).rxwm().bit(false));
        self
    }

    /// Splits the `Serial` abstraction into a transmitter and a
    /// receiver half
    pub fn split(self) -> (Tx<UART>, Rx<UART>) {
        (
            Tx {
                uart: unsafe { mem::zeroed() },
            },
            Rx { uart: self.uart },
        )
    }

    /// Releases the UART peripheral and associated pins
    pub fn free(self) -> (UART, (TX, RX)) {
        (self.uart, self.pins)
    }
}

impl<UART: UartX> serial::Read<u8> for Rx<UART> {
    type Error = Infallible;

    fn read(&mut self) -> nb::Result<u8, Infallible> {
        let rxdata = self.uart.rxdata().read();

        if rxdata.empty().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits() as u8)
        }
    }
}

impl<UART: UartX> serial::Write<u8> for Tx<UART> {
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> nb::Result<(), Infallible> {
        let txdata = self.uart.txdata().read();

        if txdata.full().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            unsafe {
                self.uart.txdata().write(|w| w.data().bits(byte));
            }
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Infallible> {
        if self.uart.ip().read().txwm().bit_is_set() {
            // FIFO count is below the receive watermark (1)
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

// Backward compatibility
impl<TX, RX> Serial<Uart0, (TX, RX)> {
    /// Configures a UART peripheral to provide serial communication
    #[deprecated(note = "Please use Serial::new function instead")]
    pub fn uart0(uart: Uart0, pins: (TX, RX), baud_rate: Bps, clocks: Clocks) -> Self
    where
        TX: TxPin<Uart0>,
        RX: RxPin<Uart0>,
    {
        Self::new(uart, pins, baud_rate, clocks)
    }
}

#[cfg(feature = "g002")]
impl<TX, RX> Serial<Uart1, (TX, RX)> {
    /// Configures a UART peripheral to provide serial communication
    #[deprecated(note = "Please use Serial::new function instead")]
    pub fn uart1(uart: Uart1, pins: (TX, RX), baud_rate: Bps, clocks: Clocks) -> Self
    where
        TX: TxPin<Uart1>,
        RX: RxPin<Uart1>,
    {
        Self::new(uart, pins, baud_rate, clocks)
    }
}
