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

use core::ops::Deref;
use e310x::{uart0, Uart0, Uart1};
use embedded_hal_nb::serial::{ErrorKind, ErrorType, Read, Write};
use nb;

use crate::{clock::Clocks, time::Bps};

/// TX pin
pub trait TxPin<UART>: private::Sealed {}

/// RX pin
pub trait RxPin<UART>: private::Sealed {}

/// UartX trait extends the UART peripheral
pub trait UartX: Deref<Target = uart0::RegisterBlock> + private::Sealed {
    /// Steals the UART peripheral
    ///
    /// # Safety
    ///
    /// Using this function may break the guarantees of the singleton pattern.
    unsafe fn steal() -> Self;
}

mod impl_uart {
    use super::{RxPin, TxPin, Uart0, Uart1, UartX};
    use crate::gpio::{gpio0, IOF0};
    // UART0
    impl UartX for Uart0 {
        unsafe fn steal() -> Self {
            Uart0::steal()
        }
    }
    impl<T> TxPin<Uart0> for gpio0::Pin17<IOF0<T>> {}
    impl<T> RxPin<Uart0> for gpio0::Pin16<IOF0<T>> {}

    // UART1
    impl UartX for Uart1 {
        unsafe fn steal() -> Self {
            Uart1::steal()
        }
    }
    #[cfg(feature = "g002")]
    impl<T> TxPin<Uart1> for gpio0::Pin18<IOF0<T>> {}
    #[cfg(feature = "g002")]
    impl<T> RxPin<Uart1> for gpio0::Pin23<IOF0<T>> {}
}

/// Serial receiver half
pub struct Rx<UART, PIN> {
    uart: UART,
    pin: PIN,
}

impl<UART, PIN> Rx<UART, PIN> {
    /// Releases the UART peripheral and associated pin
    pub fn free(self) -> (UART, PIN) {
        (self.uart, self.pin)
    }
}

impl<UART: UartX, PIN: RxPin<UART>> ErrorType for Rx<UART, PIN> {
    type Error = ErrorKind;
}

impl<UART: UartX, PIN: RxPin<UART>> Read for Rx<UART, PIN> {
    fn read(&mut self) -> nb::Result<u8, ErrorKind> {
        let rxdata = self.uart.rxdata().read();

        if rxdata.empty().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits())
        }
    }
}

/// Serial transmitter half
pub struct Tx<UART, PIN> {
    uart: UART,
    pin: PIN,
}

impl<UART, PIN> Tx<UART, PIN> {
    /// Releases the UART peripheral and associated pin
    pub fn free(self) -> (UART, PIN) {
        (self.uart, self.pin)
    }
}

impl<UART: UartX, PIN: TxPin<UART>> ErrorType for Tx<UART, PIN> {
    type Error = ErrorKind;
}

impl<UART: UartX, PIN: TxPin<UART>> Write for Tx<UART, PIN> {
    fn write(&mut self, byte: u8) -> nb::Result<(), ErrorKind> {
        let txdata = self.uart.txdata().read();

        if txdata.full().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            self.uart.txdata().write(|w| unsafe { w.data().bits(byte) });
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), ErrorKind> {
        if self.uart.ip().read().txwm().bit_is_set() {
            // FIFO count is below the receive watermark (1)
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// Serial abstraction
pub struct Serial<UART, TX, RX> {
    uart: UART,
    tx: Tx<UART, TX>,
    rx: Rx<UART, RX>,
}

impl<UART: UartX, TX: TxPin<UART>, RX: RxPin<UART>> Serial<UART, TX, RX> {
    /// Configures a UART peripheral to provide serial communication
    pub fn new(uart: UART, pins: (TX, RX), baud_rate: Bps, clocks: Clocks) -> Self {
        let div = clocks.tlclk().0 / baud_rate.0 - 1;
        unsafe {
            uart.ie().write(|w| w.txwm().bit(false).rxwm().bit(false));
            uart.div().write(|w| w.bits(div));
            uart.txctrl()
                .write(|w| w.counter().bits(1).enable().bit(true));
            uart.rxctrl().write(|w| w.enable().bit(true));
        }

        let tx = Tx {
            uart: unsafe { UART::steal() },
            pin: pins.0,
        };
        let rx = Rx {
            uart: unsafe { UART::steal() },
            pin: pins.1,
        };

        Serial { uart, tx, rx }
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

    /// Splits the [`Serial`] abstraction into a transmitter and a receiver half
    pub fn split(self) -> (Tx<UART, TX>, Rx<UART, RX>) {
        (self.tx, self.rx)
    }

    /// Releases the UART peripheral and associated pins
    pub fn free(self) -> (UART, (TX, RX)) {
        (self.uart, (self.tx.pin, self.rx.pin))
    }
}

impl<UART, TX, RX> ErrorType for Serial<UART, TX, RX> {
    type Error = ErrorKind;
}

impl<UART: UartX, TX, RX: RxPin<UART>> Read for Serial<UART, TX, RX> {
    fn read(&mut self) -> nb::Result<u8, ErrorKind> {
        self.rx.read()
    }
}

impl<UART: UartX, TX: TxPin<UART>, RX> Write for Serial<UART, TX, RX> {
    fn write(&mut self, byte: u8) -> nb::Result<(), ErrorKind> {
        self.tx.write(byte)
    }

    fn flush(&mut self) -> nb::Result<(), ErrorKind> {
        self.tx.flush()
    }
}

// seal the "private" traits
mod private {
    use crate::gpio::{gpio0, IOF0};
    use e310x::{Uart0, Uart1};

    pub trait Sealed {}

    impl Sealed for Uart0 {}
    impl<T> Sealed for gpio0::Pin17<IOF0<T>> {}
    impl<T> Sealed for gpio0::Pin16<IOF0<T>> {}

    impl Sealed for Uart1 {}
    #[cfg(feature = "g002")]
    impl<T> Sealed for gpio0::Pin18<IOF0<T>> {}
    #[cfg(feature = "g002")]
    impl<T> Sealed for gpio0::Pin23<IOF0<T>> {}
}
