//! I2C Master Interface
//!
//! The SiFive Inter-Integrated Circuit (I2C) Master Interface
//! is based on OpenCores® I2C Master Core.
//!
//! You can use the [`I2c`] interface with these I2C instances
//!
//! # I2C0
//! - SDA: Pin 12 IOF0
//! - SCL: Pin 13 IOF0
//! - Interrupt::I2C0

use crate::{clock::Clocks, time::Bps};
use core::ops::Deref;
use e310x::{i2c0, I2c0};
use e310x::{
    interrupt::{ExternalInterrupt, Priority},
    Plic,
};
use embedded_hal::i2c::{self, ErrorKind, ErrorType, NoAcknowledgeSource, Operation};

/// SDA pin
pub trait SdaPin<I2C>: private::Sealed {}

/// SCL pin
pub trait SclPin<I2C>: private::Sealed {}

/// I2cX trait extends the I2C peripheral
pub trait I2cX: Deref<Target = i2c0::RegisterBlock> + private::Sealed {}

mod i2c_impl {
    use super::{I2c0, I2cX, SclPin, SdaPin};
    use crate::gpio::{gpio0, IOF0};

    /// I2C0
    impl I2cX for I2c0 {}
    impl<T> SdaPin<I2c0> for gpio0::Pin12<IOF0<T>> {}
    impl<T> SclPin<I2c0> for gpio0::Pin13<IOF0<T>> {}
}

/// Transmission speed
pub enum Speed {
    /// 100Kbps
    Normal,

    /// 400Kbps
    Fast,

    /// Custom speed
    Custom(Bps),
}

/// I2C abstraction
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

impl<I2C: I2cX, SDA, SCL> I2c<I2C, (SDA, SCL)> {
    /// Configures an I2C peripheral
    pub fn new(i2c: I2C, sda: SDA, scl: SCL, speed: Speed, clocks: Clocks) -> Self
    where
        SDA: SdaPin<I2C>,
        SCL: SclPin<I2C>,
    {
        // Calculate prescaler value
        let desired_speed = match speed {
            Speed::Normal => 100_000,
            Speed::Fast => 400_000,
            Speed::Custom(bps) => bps.0,
        };
        let clock = clocks.tlclk().0;
        assert!(desired_speed * 5 <= clock);
        let prescaler = clock / (5 * desired_speed) - 1;
        assert!(prescaler < (1 << 16));

        // Turn off i2c
        i2c.ctr().write(|w| w.en().clear_bit().ien().clear_bit());

        // Set prescaler
        let prescaler_lo = (prescaler & 0xff) as u8;
        let prescaler_hi = ((prescaler >> 8) & 0xff) as u8;
        i2c.prer_lo()
            .write(|w| unsafe { w.value().bits(prescaler_lo) });
        i2c.prer_hi()
            .write(|w| unsafe { w.value().bits(prescaler_hi) });

        // Turn on i2c
        i2c.ctr().write(|w| w.en().set_bit());

        Self {
            i2c,
            pins: (sda, scl),
        }
    }
}

impl<I2C, PINS> I2c<I2C, PINS> {
    /// Releases the I2C peripheral and associated pins
    pub fn free(self) -> (I2C, PINS) {
        (self.i2c, self.pins)
    }
}

impl<I2C: I2cX, PINS> I2c<I2C, PINS> {
    /// Enables the external interrupt source for the I2C.
    ///
    /// # Note
    /// This function enables the external interrupt source in the PLIC,
    /// but does not enable the PLIC peripheral itself. To enable the plic peripheral
    /// you must call [`Plic::enable()`](riscv-peripheral::plic::enables::ENABLES::enable).
    ///
    /// # Safety
    /// Enabling an interrupt source can break mask-based critical sections.
    #[inline]
    pub unsafe fn enable_exti(&mut self, plic: &Plic) {
        let ctx = plic.ctx0();
        ctx.enables().enable(ExternalInterrupt::I2C0);
    }

    /// Disables the external interrupt source for the pin.
    #[inline]
    pub fn disable_exti(&mut self, plic: &Plic) {
        let ctx = plic.ctx0();
        ctx.enables().disable(ExternalInterrupt::I2C0);
    }

    /// Returns whether the external interrupt source for the pin is enabled.
    #[inline]
    pub fn is_exti_enabled(&self, plic: &Plic) -> bool {
        let ctx = plic.ctx0();
        ctx.enables().is_enabled(ExternalInterrupt::I2C0)
    }

    /// Sets the external interrupt source priority.
    ///
    /// # Safety
    ///
    /// Changing the priority level can break priority-based critical sections.
    #[inline]
    pub unsafe fn set_exti_priority(&mut self, plic: &Plic, priority: Priority) {
        let priorities = plic.priorities();
        priorities.set_priority(ExternalInterrupt::I2C0, priority);
    }

    /// Returns the external interrupt source priority.
    #[inline]
    pub fn get_exti_priority(&self, plic: &Plic) -> Priority {
        let priorities = plic.priorities();
        priorities.get_priority(ExternalInterrupt::I2C0)
    }

    /// Enable the I2C interrupt bit in the control register.
    ///
    /// # Note
    /// This function also writes in the I2C enable register as both the interrupt and the I2C peripheral
    /// need to be set at the same time.
    ///
    /// # Note
    /// This function does not enable the interrupt in the PLIC, it only sets the
    /// interrupt enable bit in the I2C peripheral. You must call
    /// [`enable_exti()`](Self::enable_exti) to enable the interrupt in
    /// the PLIC.
    #[inline]
    pub fn enable_interrupt(&mut self) {
        self.i2c.ctr().modify(|r, w| {
            w.en().bit(r.en().bit_is_set());
            w.ien().set_bit()
        });
    }

    /// Disable the I2C interrupt enable bit in the control register.
    ///
    /// # Note
    /// This function writes in the I2C enable register as both the interrupt and the I2C peripheral
    /// need to be set at the same time.
    #[inline]
    pub fn disable_interrupt(&mut self) {
        self.i2c.ctr().modify(|r, w| {
            w.en().bit(r.en().bit_is_set());
            w.ien().clear_bit()
        });
    }

    /// Clear the interrupt flag in the control register.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        self.i2c.cr().write(|w| w.iack().set_bit());
    }

    /// Returns true if the interrupt flag is set.
    pub fn is_interrupt_pending(&self) -> bool {
        self.i2c.sr().read().if_().bit_is_set()
    }

    /// Check if the I2C interrupt is enabled.
    #[inline]
    pub fn is_interrupt_enabled(&self) -> bool {
        self.i2c.ctr().read().ien().bit_is_set()
    }

    /// Read the status register.
    pub(crate) fn read_sr(&self) -> i2c0::sr::R {
        self.i2c.sr().read()
    }

    /// Reset the I2C peripheral.
    pub(crate) fn reset(&mut self) {
        self.clear_interrupt();
    }

    /// Set the stop bit in the control register.
    /// A stop condition will be sent on the bus.
    pub(crate) fn set_stop(&self) {
        self.i2c.cr().write(|w| w.sto().set_bit());
    }

    /// Set the next byte to be transmitted to the I2C slave device.
    ///
    /// # Note
    ///
    /// This function does not start the transmission. You must call
    /// [`Self::write_to_slave`] to start the transmission.
    pub(crate) fn write_txr(&self, byte: u8) {
        self.i2c.txr_rxr().write(|w| unsafe { w.data().bits(byte) });
    }

    /// Read the last byte received from the I2C slave device.
    pub(crate) fn read_rxr(&self) -> u8 {
        self.i2c.txr_rxr().read().data().bits()
    }

    /// Trigger a write to the slave device.
    /// This function should be called after writing to the transmit register.
    ///
    /// # Note
    ///
    /// This function does not block until the write is complete. You must call
    /// [`Self::wait_for_write`] to wait for the write to complete.
    pub(crate) fn trigger_write(&self, start: bool, stop: bool) {
        self.i2c
            .cr()
            .write(|w| w.sta().bit(start).wr().set_bit().sto().bit(stop));
    }

    /// Trigger a read from the slave device.
    /// This function should be called before reading the receive register.
    ///
    /// # Note
    ///
    /// This function does not block until the read is complete. You must call
    /// [`Self::wait_for_read`] to wait for the read to complete.
    pub(crate) fn trigger_read(&self, ack: bool, stop: bool) {
        self.i2c
            .cr()
            .write(|w| w.rd().set_bit().ack().bit(ack).sto().bit(stop));
    }

    /// Check if the I2C peripheral is idle.
    pub(crate) fn is_idle(&self) -> bool {
        !self.read_sr().busy().bit_is_set()
    }

    /// Blocking version of [`Self::is_idle`].
    fn wait_idle(&self) {
        while !self.is_idle() {}
    }

    /// Acknowledge an interrupt.
    ///
    /// # Errors
    ///
    /// In case of arbitration loss, a stop condition is sent
    /// and an [`ErrorKind::ArbitrationLoss`] is returned.
    pub(crate) fn ack_interrupt(&self) -> nb::Result<(), ErrorKind> {
        let sr = self.read_sr();
        if sr.tip().bit_is_clear() {
            if sr.al().bit_is_set() {
                self.set_stop();
                Err(nb::Error::Other(ErrorKind::ArbitrationLoss))
            } else {
                Ok(())
            }
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Wait for a read operation to complete.
    ///
    /// # Errors
    ///
    /// In case of arbitration loss it waits until the bus is idle
    /// before returning an [`ErrorKind::ArbitrationLoss`] error.
    fn wait_for_read(&self) -> Result<(), ErrorKind> {
        nb::block!(self.ack_interrupt())
    }

    /// Wait for a write operation to complete.
    ///
    /// # Errors
    ///
    /// If the slave device does not acknowledge the write, a stop condition
    /// is sent and an [`ErrorKind::NoAcknowledge`] is returned.
    ///
    /// In case of arbitration loss it waits until the bus is idle
    /// before returning an [`ErrorKind::ArbitrationLoss`] error.
    fn wait_for_write(&self, source: NoAcknowledgeSource) -> Result<(), ErrorKind> {
        nb::block!(self.ack_interrupt())?;
        if self.read_sr().rx_ack().bit_is_set() {
            self.set_stop();
            Err(ErrorKind::NoAcknowledge(source))
        } else {
            Ok(())
        }
    }
}

const FLAG_READ: u8 = 1;
const FLAG_WRITE: u8 = 0;

impl<I2C: I2cX, PINS> ErrorType for I2c<I2C, PINS> {
    type Error = ErrorKind;
}

impl<I2C: I2cX, PINS> i2c::I2c for I2c<I2C, PINS> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        let n_ops = operations.len();
        if n_ops == 0 {
            return Ok(());
        }

        self.wait_idle();
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
                    self.wait_for_write(NoAcknowledgeSource::Address)?;
                    last_op_was_read = false;

                    // Write bytes
                    let n_bytes = bytes.len();
                    for (j, byte) in bytes.iter().enumerate() {
                        self.write_txr(*byte);
                        self.trigger_write(false, (i == n_ops - 1) && (j == n_bytes - 1));
                        self.wait_for_write(NoAcknowledgeSource::Data)?;
                    }
                }
                Operation::Read(buffer) => {
                    // Send read command
                    self.write_txr((address << 1) + FLAG_READ);
                    self.trigger_write(!last_op_was_read, false);
                    self.wait_for_write(NoAcknowledgeSource::Address)?;
                    last_op_was_read = true;

                    // Read bytes
                    let n_bytes = buffer.len();
                    for (j, byte) in buffer.iter_mut().enumerate() {
                        self.trigger_read(j == n_bytes - 1, (i == n_ops - 1) && (j == n_bytes - 1));
                        self.wait_for_read()?;
                        *byte = self.read_rxr();
                    }
                }
            }
        }
        self.wait_idle();

        Ok(())
    }
}

mod private {
    use super::I2c0;
    use crate::gpio::{gpio0, IOF0};

    pub trait Sealed {}

    // I2C0
    impl Sealed for I2c0 {}
    impl<T> Sealed for gpio0::Pin12<IOF0<T>> {}
    impl<T> Sealed for gpio0::Pin13<IOF0<T>> {}
}
