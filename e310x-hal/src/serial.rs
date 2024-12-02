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
use embedded_hal_nb::serial;

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

impl<UART: UartX, PIN: RxPin<UART>> serial::ErrorType for Rx<UART, PIN> {
    type Error = serial::ErrorKind;
}

impl<UART: UartX, PIN: RxPin<UART>> embedded_io::ErrorType for Rx<UART, PIN> {
    type Error = embedded_io::ErrorKind;
}

impl<UART: UartX, PIN: RxPin<UART>> serial::Read for Rx<UART, PIN> {
    #[inline]
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let rxdata = self.uart.rxdata().read();

        if rxdata.empty().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits())
        }
    }
}

impl<UART: UartX, PIN: RxPin<UART>> embedded_io::Read for Rx<UART, PIN> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        buf[0] = nb::block!(serial::Read::read(self)).unwrap(); // first byte may block
        let mut count = 1;
        for byte in buf.iter_mut().skip(1) {
            match serial::Read::read(self) {
                Ok(b) => {
                    *byte = b;
                    count += 1
                }
                Err(nb::Error::WouldBlock) => break,
                _ => unreachable!(),
            }
        }
        Ok(count)
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

impl<UART: UartX, PIN: TxPin<UART>> Tx<UART, PIN> {
    /// Returns true if the transmit buffer is full
    fn is_buffer_full(&self) -> bool {
        self.uart.txdata().read().full().bit_is_set()
    }
}

impl<UART: UartX, PIN: TxPin<UART>> serial::ErrorType for Tx<UART, PIN> {
    type Error = serial::ErrorKind;
}

impl<UART: UartX, PIN: TxPin<UART>> embedded_io::ErrorType for Tx<UART, PIN> {
    type Error = embedded_io::ErrorKind;
}

impl<UART: UartX, PIN: TxPin<UART>> serial::Write for Tx<UART, PIN> {
    #[inline]
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        if self.is_buffer_full() {
            Err(nb::Error::WouldBlock)
        } else {
            self.uart.txdata().write(|w| unsafe { w.data().bits(byte) });
            Ok(())
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.uart.ip().read().txwm().bit_is_set() {
            // FIFO count is below the receive watermark (1)
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<UART: UartX, PIN: TxPin<UART>> embedded_io::WriteReady for Tx<UART, PIN> {
    #[inline]
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_buffer_full())
    }
}

impl<UART: UartX, PIN: TxPin<UART>> embedded_io::Write for Tx<UART, PIN> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        nb::block!(serial::Write::write(self, buf[0])).unwrap(); // first byte may block
        let mut count = 1;
        for byte in buf.iter().skip(1) {
            match serial::Write::write(self, *byte) {
                Ok(()) => count += 1,
                Err(nb::Error::WouldBlock) => break,
                _ => unreachable!(),
            }
        }
        Ok(count)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        nb::block!(serial::Write::flush(self)).unwrap();
        Ok(())
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

impl<UART: UartX, TX, RX> serial::ErrorType for Serial<UART, TX, RX> {
    type Error = serial::ErrorKind;
}

impl<UART: UartX, TX, RX: RxPin<UART>> serial::Read for Serial<UART, TX, RX> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.rx.read()
    }
}

impl<UART: UartX, TX: TxPin<UART>, RX> serial::Write for Serial<UART, TX, RX> {
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(byte)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
    }
}

impl<UART, TX, RX> embedded_io::ErrorType for Serial<UART, TX, RX> {
    type Error = embedded_io::ErrorKind;
}

impl<UART: UartX, TX, RX: RxPin<UART>> embedded_io::Read for Serial<UART, TX, RX> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.read(buf)
    }
}

impl<UART: UartX, TX: TxPin<UART>, RX> embedded_io::WriteReady for Serial<UART, TX, RX> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.tx.write_ready()
    }
}

impl<UART: UartX, TX: TxPin<UART>, RX> embedded_io::Write for Serial<UART, TX, RX> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
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
