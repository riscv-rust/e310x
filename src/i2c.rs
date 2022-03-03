//! I2C Master Interface
//!
//! The SiFive Inter-Integrated Circuit (I2C) Master Interface
//! is based on OpenCores® I2C Master Core.
//!
//! You can use the `I2c` interface with these I2C instances
//!
//! # I2C0
//! - SDA: Pin 12 IOF0
//! - SCL: Pin 13 IOF0
//! - Interrupt::I2C0

use crate::clock::Clocks;
use crate::gpio::{gpio0, IOF0};
use crate::time::Bps;
use core::mem;
use core::ops::Deref;
use e310x::{i2c0, I2C0};
use embedded_hal::i2c::blocking::{self as i2c, Operation};
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource};

/// SDA pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SdaPin<I2C> {}
/// SCL pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SclPin<I2C> {}

unsafe impl<T> SdaPin<I2C0> for gpio0::Pin12<IOF0<T>> {}
unsafe impl<T> SclPin<I2C0> for gpio0::Pin13<IOF0<T>> {}

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

impl<SDA, SCL> I2c<I2C0, (SDA, SCL)> {
    /// Configures an I2C peripheral
    pub fn new(i2c: I2C0, sda: SDA, scl: SCL, speed: Speed, clocks: Clocks) -> Self
    where
        SDA: SdaPin<I2C0>,
        SCL: SclPin<I2C0>,
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
        i2c.ctr.write(|w| w.en().clear_bit().ien().clear_bit());

        // Set prescaler
        let prescaler_lo = (prescaler & 0xff) as u8;
        let prescaler_hi = ((prescaler >> 8) & 0xff) as u8;
        i2c.prer_lo
            .write(|w| unsafe { w.value().bits(prescaler_lo) });
        i2c.prer_hi
            .write(|w| unsafe { w.value().bits(prescaler_hi) });

        // Turn on i2c
        i2c.ctr.write(|w| w.en().set_bit());

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

impl<I2C: Deref<Target = i2c0::RegisterBlock>, PINS> I2c<I2C, PINS> {
    fn reset(&self) {
        // ACK pending interrupt event, clear commands
        self.write_cr(|w| w.iack().set_bit());
    }

    fn write_cr<F>(&self, f: F)
    where
        F: FnOnce(&mut i2c0::cr::W) -> &mut i2c0::cr::W,
    {
        self.i2c.cr().write(|w| unsafe {
            let mut value: u32 = 0;
            f(mem::transmute(&mut value));
            w.bits(value)
        });
    }

    fn read_sr(&self) -> i2c0::sr::R {
        self.i2c.sr().read()
    }

    fn write_byte(&self, byte: u8) {
        self.i2c.txr_rxr.write(|w| unsafe { w.data().bits(byte) });
    }

    fn read_byte(&self) -> u8 {
        self.i2c.txr_rxr.read().data().bits()
    }

    fn wait_for_interrupt(&self) -> Result<(), ErrorKind> {
        loop {
            let sr = self.read_sr();

            if sr.al().bit_is_set() {
                // Set STOP
                self.write_cr(|w| w.sto().set_bit());
                self.wait_for_complete();

                return Err(ErrorKind::ArbitrationLoss);
            }

            if sr.if_().bit_is_set() {
                // ACK the interrupt
                self.write_cr(|w| w.iack().set_bit());

                return Ok(());
            }
        }
    }

    fn wait_for_read(&self) -> Result<(), ErrorKind> {
        self.wait_for_interrupt()
    }

    fn wait_for_write(&self) -> Result<(), ErrorKind> {
        self.wait_for_interrupt()?;

        if self.read_sr().rx_ack().bit_is_set() {
            // Set STOP
            self.write_cr(|w| w.sto().set_bit());
            self.wait_for_complete();

            return Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown));
        }

        Ok(())
    }

    fn wait_for_complete(&self) {
        while self.read_sr().busy().bit_is_set() {}
    }
}

const FLAG_READ: u8 = 1;
const FLAG_WRITE: u8 = 0;

impl<I2C, PINS> ErrorType for I2c<I2C, PINS> {
    type Error = ErrorKind;
}

impl<I2C: Deref<Target = i2c0::RegisterBlock>, PINS> i2c::I2c for I2c<I2C, PINS> {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        // Write address + R
        self.write_byte((address << 1) + FLAG_READ);

        // Generate start condition and write command
        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
        self.wait_for_write()?;

        // Read bytes
        let buffer_len = buffer.len();
        for (i, byte) in buffer.iter_mut().enumerate() {
            if i != buffer_len - 1 {
                // R + ACK
                self.write_cr(|w| w.rd().set_bit().ack().clear_bit());
            } else {
                // R + NACK + STOP
                self.write_cr(|w| w.rd().set_bit().ack().set_bit().sto().set_bit());
            }
            self.wait_for_read()?;

            *byte = self.read_byte();
        }
        Ok(())
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        // Write address + W
        self.write_byte((address << 1) + FLAG_WRITE);

        // Generate start condition and write command
        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
        self.wait_for_write()?;

        // Write bytes
        for (i, byte) in bytes.iter().enumerate() {
            self.write_byte(*byte);

            if i != bytes.len() - 1 {
                self.write_cr(|w| w.wr().set_bit());
            } else {
                self.write_cr(|w| w.wr().set_bit().sto().set_bit());
            }
            self.wait_for_write()?;
        }
        Ok(())
    }

    fn write_iter<B: IntoIterator<Item = u8>>(
        &mut self,
        address: u8,
        bytes: B,
    ) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        // Write address + W
        self.write_byte((address << 1) + FLAG_WRITE);

        // Generate start condition and write command
        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
        self.wait_for_write()?;

        // Write bytes
        let mut bytes = bytes.into_iter().peekable();
        while let Some(byte) = bytes.next() {
            self.write_byte(byte);

            if bytes.peek().is_some() {
                self.write_cr(|w| w.wr().set_bit()); // all others
            } else {
                self.write_cr(|w| w.wr().set_bit().sto().set_bit()); // last one
            }
            self.wait_for_write()?;
        }
        Ok(())
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        if !bytes.is_empty() && buffer.is_empty() {
            self.write(address, bytes)
        } else if !buffer.is_empty() && bytes.is_empty() {
            self.read(address, buffer)
        } else if bytes.is_empty() && buffer.is_empty() {
            Ok(())
        } else {
            // Write address + W
            self.write_byte((address << 1) + FLAG_WRITE);

            // Generate start condition and write command
            self.write_cr(|w| w.sta().set_bit().wr().set_bit());
            self.wait_for_write()?;

            // Write bytes
            for byte in bytes {
                self.write_byte(*byte);

                self.write_cr(|w| w.wr().set_bit());
                self.wait_for_write()?;
            }

            // Write address + R
            self.write_byte((address << 1) + FLAG_READ);

            // Generate repeated start condition and write command
            self.write_cr(|w| w.sta().set_bit().wr().set_bit());
            self.wait_for_write()?;

            // Read bytes
            let buffer_len = buffer.len();
            for (i, byte) in buffer.iter_mut().enumerate() {
                if i != buffer_len - 1 {
                    // W + ACK
                    self.write_cr(|w| w.rd().set_bit().ack().clear_bit());
                } else {
                    // W + NACK + STOP
                    self.write_cr(|w| w.rd().set_bit().ack().set_bit().sto().set_bit());
                }
                self.wait_for_read()?;

                *byte = self.read_byte();
            }

            Ok(())
        }
    }

    fn write_iter_read<B: IntoIterator<Item = u8>>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        let mut bytes = bytes.into_iter().peekable();
        if bytes.peek().is_some() && buffer.is_empty() {
            self.write_iter(address, bytes)
        } else if !buffer.is_empty() && bytes.peek().is_none() {
            self.read(address, buffer)
        } else if bytes.peek().is_none() && buffer.is_empty() {
            Ok(())
        } else {
            // Write address + W
            self.write_byte((address << 1) + FLAG_WRITE);

            // Generate start condition and write command
            self.write_cr(|w| w.sta().set_bit().wr().set_bit());
            self.wait_for_write()?;

            // Write bytes
            for byte in bytes {
                self.write_byte(byte);

                self.write_cr(|w| w.wr().set_bit());
                self.wait_for_write()?;
            }

            // Write address + R
            self.write_byte((address << 1) + FLAG_READ);

            // Generate repeated start condition and write command
            self.write_cr(|w| w.sta().set_bit().wr().set_bit());
            self.wait_for_write()?;

            // Read bytes
            let buffer_len = buffer.len();
            for (i, byte) in buffer.iter_mut().enumerate() {
                if i != buffer_len - 1 {
                    // W + ACK
                    self.write_cr(|w| w.rd().set_bit().ack().clear_bit());
                } else {
                    // W + NACK + STOP
                    self.write_cr(|w| w.rd().set_bit().ack().set_bit().sto().set_bit());
                }
                self.wait_for_read()?;

                *byte = self.read_byte();
            }

            Ok(())
        }
    }

    fn transaction<'a>(
        &mut self,
        address: u8,
        operations: &mut [Operation<'a>],
    ) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        let last_op = operations.len() - 1;
        let mut last_flag = 0xFF;
        for (i, op) in operations.iter_mut().enumerate() {
            match op {
                Operation::Read(bytes) => {
                    if last_flag != FLAG_READ {
                        // Write address + R
                        self.write_byte((address << 1) + FLAG_READ);

                        // Generate repeated start condition and write command
                        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
                        self.wait_for_write()?;
                        last_flag = FLAG_READ;
                    }

                    // Read bytes
                    let last_byte = bytes.len() - 1;
                    for (j, byte) in bytes.iter_mut().enumerate() {
                        if i != last_op || j != last_byte {
                            // W + ACK
                            self.write_cr(|w| w.rd().set_bit().ack().clear_bit());
                        } else {
                            // W + NACK + STOP
                            self.write_cr(|w| w.rd().set_bit().ack().set_bit().sto().set_bit());
                        }
                        self.wait_for_read()?;

                        *byte = self.read_byte();
                    }
                }
                Operation::Write(bytes) => {
                    if last_flag != FLAG_WRITE {
                        // Write address + W
                        self.write_byte((address << 1) + FLAG_WRITE);

                        // Generate start condition and write command
                        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
                        self.wait_for_write()?;
                        last_flag = FLAG_WRITE;
                    }

                    // Write bytes
                    for (j, byte) in bytes.iter().enumerate() {
                        self.write_byte(*byte);

                        if i != last_op || j != bytes.len() - 1 {
                            self.write_cr(|w| w.wr().set_bit());
                        } else {
                            self.write_cr(|w| w.wr().set_bit().sto().set_bit());
                        }
                        self.wait_for_write()?;
                    }
                }
            }
        }

        Ok(())
    }

    fn transaction_iter<'a, O: IntoIterator<Item = Operation<'a>>>(
        &mut self,
        address: u8,
        operations: O,
    ) -> Result<(), Self::Error> {
        self.reset();

        if self.read_sr().busy().bit_is_set() {
            return Err(ErrorKind::Other);
        }

        let mut last_flag = 0xFF;
        let mut operations = operations.into_iter().peekable();
        while let Some(op) = operations.next() {
            match op {
                Operation::Read(bytes) => {
                    if last_flag != FLAG_READ {
                        // Write address + R
                        self.write_byte((address << 1) + FLAG_READ);

                        // Generate repeated start condition and write command
                        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
                        self.wait_for_write()?;
                        last_flag = FLAG_READ;
                    }

                    // Read bytes
                    let last_byte = bytes.len() - 1;
                    for (j, byte) in bytes.iter_mut().enumerate() {
                        if operations.peek().is_some() || j != last_byte {
                            // W + ACK
                            self.write_cr(|w| w.rd().set_bit().ack().clear_bit());
                        } else {
                            // W + NACK + STOP
                            self.write_cr(|w| w.rd().set_bit().ack().set_bit().sto().set_bit());
                        }
                        self.wait_for_read()?;

                        *byte = self.read_byte();
                    }
                }
                Operation::Write(bytes) => {
                    if last_flag != FLAG_WRITE {
                        // Write address + W
                        self.write_byte((address << 1) + FLAG_WRITE);

                        // Generate start condition and write command
                        self.write_cr(|w| w.sta().set_bit().wr().set_bit());
                        self.wait_for_write()?;
                        last_flag = FLAG_WRITE;
                    }

                    // Write bytes
                    for (j, byte) in bytes.iter().enumerate() {
                        self.write_byte(*byte);

                        if operations.peek().is_some() || j != bytes.len() - 1 {
                            self.write_cr(|w| w.wr().set_bit());
                        } else {
                            self.write_cr(|w| w.wr().set_bit().sto().set_bit());
                        }
                        self.wait_for_write()?;
                    }
                }
            }
        }

        Ok(())
    }
}
