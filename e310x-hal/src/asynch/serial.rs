//! Serial interface
//!
//! Implementation of the asynchronous [`embedded_io_async`] traits.
//!
//! You can use the [`Serial`] interface with these UART instances
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

use crate::serial::{Rx, RxPin, Serial, Tx, TxPin, UartX, WatermarkValue};
use core::cell::RefCell;
use core::future::poll_fn;
use core::task::{Poll, Waker};
use critical_section::Mutex;
use e310x::{Uart0, Uart1};
use embedded_hal_nb::serial;

type WakerPair = (Option<Waker>, Option<Waker>);
const N_UARTS: usize = 2;
static UART_WAKERS: Mutex<RefCell<[WakerPair; N_UARTS]>> =
    Mutex::new(RefCell::new([const { (None, None) }; N_UARTS]));

#[inline]
fn on_irq<UART: UartX>(uart: UART) {
    // Check if the Rx interrupt is enabled and pending
    if uart.ie().read().rxwm().bit_is_set() && uart.ip().read().rxwm().bit_is_set() {
        // Wake the waker if it exists
        critical_section::with(|cs| {
            let mut uartwaker = UART_WAKERS.borrow_ref_mut(cs);
            if let Some(waker) = uartwaker[UART::UART_INDEX].0.take() {
                waker.wake();
            }
        });
        // Disable the interrupt
        uart.ie().modify(|r, w| {
            w.txwm().bit(r.txwm().bit());
            w.rxwm().clear_bit()
        });
    }
    // Check if the Tx interrupt is enabled and pending
    if uart.ie().read().txwm().bit_is_set() && uart.ip().read().txwm().bit_is_set() {
        // Wake the waker if it exists
        critical_section::with(|cs| {
            let mut uartwaker = UART_WAKERS.borrow_ref_mut(cs);
            if let Some(waker) = uartwaker[UART::UART_INDEX].1.take() {
                waker.wake();
            }
        });
        // Disable the interrupt
        uart.ie().modify(|r, w| {
            w.rxwm().bit(r.rxwm().bit());
            w.txwm().clear_bit()
        });
    }
}

impl<UART: UartX, PIN: RxPin<UART>> embedded_io_async::Read for Rx<UART, PIN> {
    /// This implementation is not side-effect free on cancel
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut count = 0;

        poll_fn(|cx| {
            let read = serial::Read::read(self);
            match read {
                Ok(b) => {
                    buf[0] = b;
                    count += 1;
                    Poll::Ready(())
                }
                Err(nb::Error::WouldBlock) => {
                    // Register the waker for the UART
                    critical_section::with(|cs| {
                        let mut uartwaker = UART_WAKERS.borrow_ref_mut(cs);
                        uartwaker[UART::UART_INDEX].0 = Some(cx.waker().clone());
                    });
                    // Enable the interrupt for the UART
                    self.set_watermark(WatermarkValue::W0);
                    self.enable_interrupt();
                    Poll::Pending
                }
                _ => unreachable!(),
            }
        })
        .await;

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

impl<UART: UartX, PIN: TxPin<UART>> embedded_io_async::Write for Tx<UART, PIN> {
    /// This implementation is not side-effect free on cancel
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut count = 0;

        poll_fn(|cx| match serial::Write::write(self, buf[0]) {
            Ok(()) => {
                count += 1;
                Poll::Ready(())
            }
            Err(nb::Error::WouldBlock) => {
                // Register the waker for the UART
                critical_section::with(|cs| {
                    let mut uartwaker = UART_WAKERS.borrow_ref_mut(cs);
                    uartwaker[UART::UART_INDEX].1 = Some(cx.waker().clone());
                });
                // Enable the interrupt for the UART
                self.set_watermark(WatermarkValue::W7);
                self.enable_interrupt();
                Poll::Pending
            }
            _ => unreachable!(),
        })
        .await;

        for byte in buf.iter().skip(1) {
            match serial::Write::write(self, *byte) {
                Ok(()) => count += 1,
                Err(nb::Error::WouldBlock) => break,
                _ => unreachable!(),
            }
        }
        Ok(count)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        let mut flushed = false;
        poll_fn(|cx| {
            if flushed {
                Poll::Ready(Ok(()))
            } else {
                flushed = true;
                // Register the waker for the UART
                critical_section::with(|cs| {
                    let mut uartwaker = UART_WAKERS.borrow_ref_mut(cs);
                    uartwaker[UART::UART_INDEX].1 = Some(cx.waker().clone());
                });
                // Enable the interrupt for the UART
                self.set_watermark(WatermarkValue::W1);
                self.enable_interrupt();
                Poll::Pending
            }
        })
        .await
    }
}

impl<UART: UartX, TX, RX: RxPin<UART>> embedded_io_async::Read for Serial<UART, TX, RX> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.read(buf).await
    }
}

impl<UART: UartX, TX: TxPin<UART>, RX> embedded_io_async::Write for Serial<UART, TX, RX> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.write(buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.flush().await
    }
}

/// Interrupt Handlers
#[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::UART0)]
fn uart0_handler() {
    let uart0 = unsafe { Uart0::steal() };
    on_irq(uart0);
}

#[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::UART1)]
fn uart1_handler() {
    let uart1 = unsafe { Uart1::steal() };
    on_irq(uart1);
}
