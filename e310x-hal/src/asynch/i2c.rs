//! # I2C Async API
//! # Note
//!
//! Implementation of the Async Embedded HAL I2C functionality.
//!
use crate::i2c::{I2c, I2cX};
use core::cell::RefCell;
use core::future::poll_fn;
use core::task::{Poll, Waker};
use critical_section::Mutex;
use e310x::I2c0;
use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource, Operation};
use embedded_hal_async::i2c;

const FLAG_READ: u8 = 1;
const FLAG_WRITE: u8 = 0;
static I2C_WAKER: Mutex<RefCell<Option<Waker>>> = Mutex::new(RefCell::new(None));

impl<I2C: I2cX, PINS> I2c<I2C, PINS> {
    /// Wait until the I2C bus is idle.
    async fn wait_idle_async(&mut self) {
        poll_fn(|cx| {
            if self.is_idle() {
                Poll::Ready(())
            } else {
                // Register the waker to be notified when an interrupt occurs
                critical_section::with(|cs| {
                    let mut i2cwaker = I2C_WAKER.borrow_ref_mut(cs);
                    *i2cwaker = Some(cx.waker().clone())
                });
                // Turn on i2c interrupt
                self.enable_interrupt();
                Poll::Pending
            }
        })
        .await;
    }

    /// Acknowledge the I2C interrupt.
    async fn ack_interrupt_async(&mut self) -> Result<(), ErrorKind> {
        poll_fn(|cx| {
            let result = self.ack_interrupt();
            match result {
                Ok(()) => Poll::Ready(Ok(())),
                Err(nb::Error::WouldBlock) => {
                    // Register the waker to be notified when an interrupt occurs
                    critical_section::with(|cs| {
                        let mut i2cwaker = I2C_WAKER.borrow_ref_mut(cs);
                        *i2cwaker = Some(cx.waker().clone())
                    });
                    // Turn on i2c interrupt
                    self.enable_interrupt();
                    Poll::Pending
                }
                Err(nb::Error::Other(e)) => Poll::Ready(Err(e)),
            }
        })
        .await
    }

    /// Asynchronously wait for a write operation to complete.
    async fn wait_for_write_async(&mut self, source: NoAcknowledgeSource) -> Result<(), ErrorKind> {
        self.ack_interrupt_async().await?;
        if self.read_sr().rx_ack().bit_is_set() {
            self.set_stop();
            Err(ErrorKind::NoAcknowledge(source))
        } else {
            Ok(())
        }
    }

    /// Asynchronously wait for a read operation to complete.
    async fn wait_for_read_async(&mut self) -> Result<(), ErrorKind> {
        self.ack_interrupt_async().await
    }
}

impl<I2C: I2cX, PINS> i2c::I2c for I2c<I2C, PINS> {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let n_ops = operations.len();
        if n_ops == 0 {
            return Ok(());
        }

        self.wait_idle_async().await;
        self.reset();

        // we use this flag to detect when we need to send a (repeated) start
        let mut last_op_was_read = match &operations[0] {
            Operation::Read(_) => false,
            Operation::Write(_) => true,
        };

        for (i, operation) in operations.iter_mut().enumerate() {
            match operation {
                Operation::Write(bytes) => {
                    // Send write command
                    self.write_txr((address << 1) + FLAG_WRITE);
                    self.trigger_write(last_op_was_read, false);
                    self.wait_for_write_async(NoAcknowledgeSource::Address)
                        .await?;
                    last_op_was_read = false;

                    // Write bytes
                    let n_bytes = bytes.len();
                    for (j, byte) in bytes.iter().enumerate() {
                        self.write_txr(*byte);
                        self.trigger_write(false, (i == n_ops - 1) && (j == n_bytes - 1));
                        self.wait_for_write_async(NoAcknowledgeSource::Data).await?;
                    }
                }
                Operation::Read(buffer) => {
                    // Send read command
                    self.write_txr((address << 1) + FLAG_READ);
                    self.trigger_write(!last_op_was_read, false);
                    self.wait_for_write_async(NoAcknowledgeSource::Address)
                        .await?;
                    last_op_was_read = true;

                    // Read bytes
                    let n_bytes = buffer.len();
                    for (j, byte) in buffer.iter_mut().enumerate() {
                        self.trigger_read(j == n_bytes - 1, (i == n_ops - 1) && (j == n_bytes - 1));
                        self.wait_for_read_async().await?;
                        *byte = self.read_rxr();
                    }
                }
            }
        }
        Ok(())
    }
}

/// Interrupt Handler
#[riscv_rt::external_interrupt(e310x::interrupt::ExternalInterrupt::I2C0)]
fn i2c_handler() {
    // Wake the waker if it exists
    critical_section::with(|cs| {
        let mut i2cwaker = I2C_WAKER.borrow_ref_mut(cs);
        if let Some(waker) = i2cwaker.take() {
            waker.wake();
        }
    });
    // Disable and clear the interrupt
    let i2c = unsafe { I2c0::steal() };
    i2c.ctr().modify(|r, w| {
        w.en().bit(r.en().bit_is_set());
        w.ien().clear_bit()
    });
    i2c.cr().write(|w| w.iack().set_bit());
}
