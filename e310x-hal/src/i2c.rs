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
    /// Read the status register.
    fn read_sr(&self) -> i2c0::sr::R {
        self.i2c.sr().read()
    }

    /// Clear the interrupt flag in the control register.
    fn clear_interrupt(&self) {
        self.i2c.cr().write(|w| w.iack().set_bit());
    }

    /// Reset the I2C peripheral.
    fn reset(&self) {
        self.clear_interrupt();
    }

    /// Set the start bit in the control register. The next byte sent
    /// through the bus will be preceded by a (repeated) start condition.
    ///
    /// # Note
    ///
    /// This function does not start the transmission. You must call
    /// [`Self::write_to_slave`] to start the transmission.
    fn set_start(&self) {
        self.i2c.cr().write(|w| w.sta().set_bit());
    }

    /// Set the stop bit in the control register.
    /// A stop condition will be sent on the bus.
    fn set_stop(&self) {
        self.i2c.cr().write(|w| w.sto().set_bit());
    }

    /// Set the ACK bit in the control register. If `ack` is `true`, the
    /// I2C will **NOT** acknowledge the next byte received. If `ack`
    /// is `false`, the I2C will acknowledge the next byte received.
    fn set_ack(&self, ack: bool) {
        self.i2c.cr().write(|w| w.ack().bit(ack));
    }

    /// Set the next byte to be transmitted to the I2C slave device.
    ///
    /// # Note
    ///
    /// This function does not start the transmission. You must call
    /// [`Self::write_to_slave`] to start the transmission.
    fn write_txr(&self, byte: u8) {
        self.i2c.txr_rxr().write(|w| unsafe { w.data().bits(byte) });
    }

    /// Read the last byte received from the I2C slave device.
    fn read_rxr(&self) -> u8 {
        self.i2c.txr_rxr().read().data().bits()
    }

    /// Trigger a write to the slave device.
    /// This function should be called after writing to the transmit register.
    ///
    /// # Note
    ///
    /// This function does not block until the write is complete. You must call
    /// [`Self::wait_for_write`] to wait for the write to complete.
    fn write_to_slave(&self) {
        self.i2c.cr().write(|w| w.wr().set_bit());
    }

    /// Trigger a read from the slave device.
    /// This function should be called before reading the receive register.
    ///
    /// # Note
    ///
    /// This function does not block until the read is complete. You must call
    /// [`Self::wait_for_read`] to wait for the read to complete.
    fn read_from_slave(&self) {
        self.i2c.cr().write(|w| w.rd().set_bit());
    }

    /// Check if the I2C peripheral is idle.
    fn is_idle(&self) -> nb::Result<(), ErrorKind> {
        match self.read_sr().busy().bit_is_set() {
            true => Err(nb::Error::WouldBlock),
            false => Ok(()),
        }
    }

    /// Acknowledge an interrupt.
    ///
    /// # Errors
    ///
    /// In case of arbitration loss, a stop condition is sent
    /// and an [`ErrorKind::ArbitrationLoss`] is returned.
    fn ack_interrupt(&self) -> nb::Result<(), ErrorKind> {
        let sr = self.read_sr();

        if sr.al().bit_is_set() {
            self.set_stop();
            Err(nb::Error::Other(ErrorKind::ArbitrationLoss))
        } else if sr.if_().bit_is_set() {
            self.clear_interrupt();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Blocking version of [`Self::is_idle`].
    fn wait_idle(&self) {
        nb::block!(self.is_idle()).unwrap();
    }

    /// Wait for a read operation to complete.
    ///
    /// # Errors
    ///
    /// In case of arbitration loss it waits until the bus is idle
    /// before returning an [`ErrorKind::ArbitrationLoss`] error.
    fn wait_for_read(&self) -> Result<(), ErrorKind> {
        if let Err(e) = nb::block!(self.ack_interrupt()) {
            self.wait_idle();
            Err(e)
        } else {
            Ok(())
        }
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
    fn wait_for_write(&self) -> Result<(), ErrorKind> {
        if let Err(e) = nb::block!(self.ack_interrupt()) {
            self.wait_idle();
            Err(e)
        } else if self.read_sr().rx_ack().bit_is_set() {
            self.set_stop();
            Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown))
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
        self.wait_idle();
        self.reset();

        self.set_start();
        let mut last_op_was_read = false;
        for operation in operations.iter_mut() {
            match operation {
                Operation::Write(bytes) => {
                    if last_op_was_read {
                        self.set_start();
                        last_op_was_read = false;
                    }
                    // Send write command
                    self.write_txr((address << 1) + FLAG_WRITE);
                    self.write_to_slave();
                    self.wait_for_write()?;

                    // Write bytes
                    for byte in bytes.iter() {
                        self.write_txr(*byte);
                        self.write_to_slave();
                        self.wait_for_write()?;
                    }
                }
                Operation::Read(buffer) => {
                    if !last_op_was_read {
                        self.set_start();
                        last_op_was_read = true;
                    }
                    // Send read command
                    self.write_txr((address << 1) + FLAG_READ);
                    self.write_to_slave();
                    self.wait_for_write()?;

                    // Read bytes
                    let buffer_len = buffer.len();
                    for (i, byte) in buffer.iter_mut().enumerate() {
                        // Set ACK on all but the last byte
                        self.set_ack(i == buffer_len - 1);
                        self.read_from_slave();
                        self.wait_for_read()?;
                        *byte = self.read_rxr();
                    }
                }
            }
        }
        self.set_stop();
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
