use super::{SharedBus, SpiExclusiveDevice};
use crate::spi::{CommType, Pins, PinsFull, PinsNoCS, SpiBus, SpiConfig, SpiX, WatermarkValue};
use core::cell::RefCell;
use core::future::poll_fn;
use core::task::{Poll, Waker};
use critical_section::Mutex;
use e310x::{Qspi0, Qspi1, Qspi2};
use embassy_futures::join::join;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal_async::{
    delay::DelayNs,
    spi::{self, ErrorKind},
};

const EMPTY_WRITE_PAD: u8 = 0x00;
type WakerPair = (Option<Waker>, Option<Waker>);
const N_SPI: usize = 3;
static SPI_WAKERS: Mutex<RefCell<[WakerPair; N_SPI]>> =
    Mutex::new(RefCell::new([const { (None, None) }; N_SPI]));

/// Interrupt handler function for SPI
#[inline]
fn on_irq<SPI: SpiX>(qspi: SPI) {
    if qspi.ie().read().rxwm().bit_is_set() && qspi.ip().read().rxwm().bit_is_set() {
        // Awake the waker for the corresponding SPI index
        critical_section::with(|cs| {
            if let Some(waker) = SPI_WAKERS.borrow_ref_mut(cs)[SPI::SPI_INDEX].0.take() {
                waker.wake();
            }
        });

        // Disable the interrupt
        qspi.ie().modify(|r, w| {
            w.txwm().bit(r.txwm().bit());
            w.rxwm().clear_bit()
        });
    }

    if qspi.ie().read().txwm().bit_is_set() && qspi.ip().read().txwm().bit_is_set() {
        // Awake the waker for the corresponding SPI index
        critical_section::with(|cs| {
            if let Some(waker) = SPI_WAKERS.borrow_ref_mut(cs)[SPI::SPI_INDEX].1.take() {
                waker.wake();
            }
        });

        // Disable the interrupt
        qspi.ie().modify(|r, w| {
            w.rxwm().bit(r.rxwm().bit());
            w.txwm().clear_bit()
        });
    }
}

impl<SPI: SpiX, PINS: Pins<SPI>> SpiBus<SPI, PINS> {
    /// Create a new [`SpiExclusiveDeviceAsync`] for exclusive use on this bus
    pub fn new_device_async<D: DelayNs>(
        self,
        config: &SpiConfig,
        delay: D,
    ) -> SpiExclusiveDevice<SPI, PINS, D> {
        SpiExclusiveDevice::new(self, config, delay)
    }
}

impl<SPI: SpiX, PINS: PinsNoCS<SPI>> SpiBus<SPI, PINS> {
    /// Create a [`SharedBus`] for use with multiple devices.
    pub fn shared_async<M: RawMutex>(spi: SPI, pins: PINS) -> SharedBus<M, SPI, PINS> {
        SharedBus::new(Self::new(spi, pins))
    }
}

impl<SPI: SpiX, PINS> SpiBus<SPI, PINS> {
    /// Read a single byte from the SPI bus.
    ///
    /// This function will wait if the RX FIFO is empty.
    async fn read_input_async(&self) -> Result<u8, ErrorKind> {
        // Poll the input until data is available
        poll_fn(|cx| match self.read_input() {
            Ok(data) => Poll::Ready(Ok(data)),
            Err(nb::Error::WouldBlock) => {
                // Register the waker to be notified when an interrupt occurs
                critical_section::with(|cs| {
                    let spiwaker = &mut SPI_WAKERS.borrow_ref_mut(cs)[SPI::SPI_INDEX].0;
                    *spiwaker = Some(cx.waker().clone());
                });
                self.set_watermark(CommType::Rx, WatermarkValue::W0);
                self.enable_interrupt(CommType::Rx);
                Poll::Pending
            }
            Err(nb::Error::Other(e)) => Poll::Ready(Err(e)),
        })
        .await
    }

    /// Write a single byte to the SPI bus.
    ///
    /// This function will wait if the TX FIFO is full.
    async fn write_output_async(&self, word: u8) -> Result<(), ErrorKind> {
        // Poll the output until it is ready to accept data
        poll_fn(|cx| match self.write_output(word) {
            Ok(()) => Poll::Ready(Ok(())),
            Err(nb::Error::WouldBlock) => {
                // Register the waker to be notified when an interrupt occurs
                critical_section::with(|cs| {
                    let spiwaker = &mut SPI_WAKERS.borrow_ref_mut(cs)[SPI::SPI_INDEX].1;
                    *spiwaker = Some(cx.waker().clone())
                });
                self.set_watermark(CommType::Tx, WatermarkValue::W7);
                self.enable_interrupt(CommType::Tx);
                Poll::Pending
            }
            Err(nb::Error::Other(e)) => Poll::Ready(Err(e)),
        })
        .await
    }

    /// Wake the read task
    fn wake_read(&self, wakers: &Mutex<RefCell<WakerPair>>) {
        critical_section::with(|cs| {
            let read_waker = &mut wakers.borrow_ref_mut(cs).0;
            if let Some(waker) = read_waker.take() {
                waker.wake();
            }
        });
    }

    ///Wake the write task
    fn wake_write(&self, wakers: &Mutex<RefCell<WakerPair>>) {
        critical_section::with(|cs| {
            let write_waker = &mut wakers.borrow_ref_mut(cs).1;
            if let Some(waker) = write_waker.take() {
                waker.wake();
            }
        });
    }

    /// Wait for a read to occur
    async fn wait_for_read(&self, wakers: &Mutex<RefCell<WakerPair>>) {
        let mut wait = false;
        poll_fn(|cx| {
            if wait {
                Poll::Ready(())
            } else {
                wait = true;
                critical_section::with(|cs| {
                    let write_waker = &mut wakers.borrow_ref_mut(cs).1;
                    *write_waker = Some(cx.waker().clone());
                });
                Poll::Pending
            }
        })
        .await;
    }

    /// Wait for a write to occur
    async fn wait_for_write(&self, wakers: &Mutex<RefCell<WakerPair>>) {
        let mut wait = false;
        poll_fn(|cx| {
            if wait {
                Poll::Ready(())
            } else {
                wait = true;
                critical_section::with(|cs| {
                    let read_waker = &mut wakers.borrow_ref_mut(cs).0;
                    *read_waker = Some(cx.waker().clone());
                });
                Poll::Pending
            }
        })
        .await;
    }
}

impl<SPI: SpiX, PINS: PinsFull<SPI>> spi::SpiBus for SpiBus<SPI, PINS> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let iwrite = Mutex::new(RefCell::new(0));
        let iread = Mutex::new(RefCell::new(0));
        let len = words.len();
        let wakers: Mutex<RefCell<WakerPair>> = Mutex::new(RefCell::new((None, None)));

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        let writer = async {
            let mut write_count = 0;
            let mut read_count = 0;

            while write_count < len {
                match self.write_output_async(EMPTY_WRITE_PAD).await {
                    Ok(()) => {
                        critical_section::with(|cs| {
                            *iwrite.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if read_count < write_count {
                            self.wake_read(&wakers);
                        }
                        // Wait for a read to occur if the RX FIFO is full
                        if write_count - read_count == 8 {
                            self.wait_for_read(&wakers).await;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };

        let reader = async {
            let mut write_count = 0;
            let mut read_count = 0;

            critical_section::with(|cs| {
                write_count = *iwrite.borrow_ref(cs);
            });

            while read_count < len {
                if write_count < read_count {
                    self.wait_for_write(&wakers).await;
                }
                // Read the data
                match self.read_input_async().await {
                    Ok(data) => {
                        unsafe { *words.get_unchecked_mut(read_count) = data };
                        critical_section::with(|cs| {
                            *iread.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if write_count - read_count < 8 {
                            self.wake_write(&wakers);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };
        let (write_result, read_result) = join(reader, writer).await;
        write_result.and(read_result)
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let iwrite = Mutex::new(RefCell::new(0));
        let iread = Mutex::new(RefCell::new(0));
        let len = words.len();
        let wakers: Mutex<RefCell<WakerPair>> = Mutex::new(RefCell::new((None, None)));

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        let writer = async {
            let mut write_count = 0;
            let mut read_count = 0;

            while write_count < len {
                let byte = unsafe { words.get_unchecked(write_count) };
                match self.write_output_async(*byte).await {
                    Ok(()) => {
                        critical_section::with(|cs| {
                            *iwrite.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if read_count < write_count {
                            self.wake_read(&wakers);
                        }
                        // Wait for a read to occur if the RX FIFO is full
                        if write_count - read_count == 8 {
                            self.wait_for_read(&wakers).await;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };

        let reader = async {
            let mut write_count = 0;
            let mut read_count = 0;

            critical_section::with(|cs| {
                write_count = *iwrite.borrow_ref(cs);
            });

            while read_count < len {
                if write_count < read_count {
                    self.wait_for_write(&wakers).await;
                }
                // Read the data
                match self.read_input_async().await {
                    Ok(_) => {
                        critical_section::with(|cs| {
                            *iread.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if write_count - read_count < 8 {
                            self.wake_write(&wakers);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };
        let (write_result, read_result) = join(reader, writer).await;
        write_result.and(read_result)
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let iwrite = Mutex::new(RefCell::new(0));
        let iread = Mutex::new(RefCell::new(0));
        let max_len = read.len().max(write.len());
        let wakers: Mutex<RefCell<WakerPair>> = Mutex::new(RefCell::new((None, None)));

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        let writer = async {
            let mut write_count = 0;
            let mut read_count = 0;

            while write_count < max_len {
                let byte = write.get(write_count).unwrap_or(&EMPTY_WRITE_PAD);
                match self.write_output_async(*byte).await {
                    Ok(()) => {
                        critical_section::with(|cs| {
                            *iwrite.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if read_count < write_count {
                            self.wake_read(&wakers);
                        }
                        // Wait for a read to occur if the RX FIFO is full
                        if write_count - read_count == 8 {
                            self.wait_for_read(&wakers).await;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };

        let reader = async {
            let mut write_count = 0;
            let mut read_count = 0;

            critical_section::with(|cs| {
                write_count = *iwrite.borrow_ref(cs);
            });

            while read_count < max_len {
                if write_count < read_count {
                    self.wait_for_write(&wakers).await;
                }
                // Read the data
                match self.read_input_async().await {
                    Ok(data) => {
                        if let Some(byte) = read.get_mut(read_count) {
                            *byte = data;
                        }
                        critical_section::with(|cs| {
                            *iread.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if write_count - read_count < 8 {
                            self.wake_write(&wakers);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };
        let (write_result, read_result) = join(reader, writer).await;
        write_result.and(read_result)
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let iwrite = Mutex::new(RefCell::new(0));
        let iread = Mutex::new(RefCell::new(0));
        let len = words.len();
        let wakers: Mutex<RefCell<WakerPair>> = Mutex::new(RefCell::new((None, None)));
        let words = Mutex::new(RefCell::new(words));

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        let writer = async {
            let mut write_count = 0;
            let mut read_count = 0;

            while write_count < len {
                let byte = critical_section::with(|cs| unsafe {
                    *words.borrow_ref(cs).get_unchecked(write_count)
                });
                match self.write_output_async(byte).await {
                    Ok(()) => {
                        critical_section::with(|cs| {
                            *iwrite.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if read_count < write_count {
                            self.wake_read(&wakers);
                        }
                        // Wait for a read to occur if the RX FIFO is full
                        if write_count - read_count == 8 {
                            self.wait_for_read(&wakers).await;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };

        let reader = async {
            let mut write_count = 0;
            let mut read_count = 0;

            critical_section::with(|cs| {
                write_count = *iwrite.borrow_ref(cs);
            });

            while read_count < len {
                if write_count < read_count {
                    self.wait_for_write(&wakers).await;
                }
                // Read the data
                match self.read_input_async().await {
                    Ok(data) => {
                        critical_section::with(|cs| {
                            unsafe {
                                *words.borrow_ref_mut(cs).get_unchecked_mut(read_count) = data
                            };
                            *iread.borrow_ref_mut(cs) += 1;
                            write_count = *iwrite.borrow_ref(cs);
                            read_count = *iread.borrow_ref(cs);
                        });
                        if write_count - read_count < 8 {
                            self.wake_write(&wakers);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        };

        let (write_result, read_result) = join(reader, writer).await;
        write_result.and(read_result)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        // Empty RX FIFO
        self.wait_for_rxfifo();

        let mut flushed = false;
        // Wait for the TX FIFO to be empty
        poll_fn(|cx| {
            if flushed {
                Poll::Ready(())
            } else {
                flushed = true;
                // Register the waker to be notified when an interrupt occurs
                critical_section::with(|cs| {
                    let spiwaker = &mut SPI_WAKERS.borrow_ref_mut(cs)[SPI::SPI_INDEX].1;
                    *spiwaker = Some(cx.waker().clone())
                });
                self.set_watermark(CommType::Tx, WatermarkValue::W1);
                self.enable_interrupt(CommType::Tx);
                Poll::Pending
            }
        })
        .await;

        Ok(())
    }
}

/// Interrupt Handlers
#[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::QSPI0)]
fn qspi0_handler() {
    let qspi0 = unsafe { Qspi0::steal() };
    on_irq(qspi0);
}
#[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::QSPI1)]
fn qspi1_handler() {
    let qspi1 = unsafe { Qspi1::steal() };
    on_irq(qspi1);
}
#[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::QSPI2)]
fn qspi2_handler() {
    let qspi2 = unsafe { Qspi2::steal() };
    on_irq(qspi2);
}
