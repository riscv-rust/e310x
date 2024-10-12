use embedded_hal::spi::{self, ErrorKind, ErrorType, Phase, Polarity};
use embedded_hal_nb::spi::FullDuplex;

use super::{Pins, PinsFull, PinsNoCS, SharedBus, SpiConfig, SpiExclusiveDevice, SpiX};

const EMPTY_WRITE_PAD: u8 = 0x00;

/// SPI bus abstraction
pub struct SpiBus<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

impl<SPI, PINS> SpiBus<SPI, PINS> {
    /// Releases the SPI peripheral and associated pins
    pub fn release(self) -> (SPI, PINS) {
        (self.spi, self.pins)
    }
}

impl<SPI: SpiX, PINS> SpiBus<SPI, PINS> {
    /// Starts frame by flagging CS assert, unless CSMODE = OFF
    pub(crate) fn start_frame(&mut self) {
        if !self.spi.csmode().read().mode().is_off() {
            self.spi.csmode().write(|w| w.mode().hold());
        }
    }

    /// Finishes frame flagging CS deassert, unless CSMODE = OFF
    pub(crate) fn end_frame(&mut self) {
        if !self.spi.csmode().read().mode().is_off() {
            self.spi.csmode().write(|w| w.mode().auto());
        }
    }

    /// Read a single byte from the SPI bus.
    ///
    /// This function will return `nb::Error::WouldBlock` if the RX FIFO is empty.
    fn read_input(&self) -> nb::Result<u8, ErrorKind> {
        let rxdata = self.spi.rxdata().read();
        if rxdata.empty().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits())
        }
    }

    /// Write a single byte to the SPI bus.
    ///
    /// This function will return `nb::Error::WouldBlock` if the TX FIFO is full.
    fn write_output(&self, word: u8) -> nb::Result<(), ErrorKind> {
        if self.spi.txdata().read().full().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            self.spi.txdata().write(|w| unsafe { w.data().bits(word) });
            Ok(())
        }
    }

    /// Wait for RX FIFO to be empty
    ///
    /// # Note
    ///
    /// Data in the RX FIFO (if any) will be lost.
    fn wait_for_rxfifo(&self) {
        // Ensure that RX FIFO is empty
        while self.read_input().is_ok() {}
    }
}

impl<SPI: SpiX, PINS: Pins<SPI>> SpiBus<SPI, PINS> {
    /// Construct the [`SpiBus`] for use with [`SpiSharedDevice`](super::SpiSharedDevice)
    /// or [`SpiExclusiveDevice`]
    pub fn new(spi: SPI, pins: PINS) -> Self {
        Self { spi, pins }
    }

    /// Create a new [`SpiExclusiveDevice`] for exclusive use on this bus
    pub fn new_device(self, config: &SpiConfig) -> SpiExclusiveDevice<SPI, PINS> {
        SpiExclusiveDevice::new(self, config)
    }

    /// Configure the [`SpiBus`] with given [`SpiConfig`]
    ///
    /// # Safety
    ///
    /// The provided CS index must be valid for the given SPI peripheral.
    pub(crate) unsafe fn configure(&mut self, config: &SpiConfig, cs_index: Option<u32>) {
        self.spi
            .sckdiv()
            .write(|w| unsafe { w.div().bits(config.clock_divisor as u16) });

        if let Some(index) = cs_index {
            self.spi.csid().write(|w| unsafe { w.bits(index) });
        }
        self.spi
            .csmode()
            .write(|w| w.mode().variant(config.cs_mode));

        // Set CS pin polarity to high
        self.spi.csdef().reset();

        // Set SPI mode
        let phase = config.mode.phase == Phase::CaptureOnSecondTransition;
        let polarity = config.mode.polarity == Polarity::IdleHigh;
        self.spi
            .sckmode()
            .write(|w| w.pha().bit(phase).pol().bit(polarity));

        self.spi.fmt().write(|w| unsafe {
            w.proto().single();
            w.endian().big(); // Transmit most-significant bit (MSB) first
            w.dir().rx();
            w.len().bits(8)
        });

        // Set watermark levels
        self.spi
            .txmark()
            .write(|w| unsafe { w.txmark().bits(config.txmark) });
        self.spi
            .rxmark()
            .write(|w| unsafe { w.rxmark().bits(config.rxmark) });

        // set delays
        self.spi.delay0().write(|w| unsafe {
            w.cssck().bits(config.delays.cssck); // delay between assert and clock
            w.sckcs().bits(config.delays.sckcs) // delay between clock and de-assert
        });
        self.spi.delay1().write(|w| unsafe {
            w.intercs().bits(config.delays.intercs); // delay between CS re-assets
            w.interxfr().bits(config.delays.interxfr) // intra-frame delay without CS re-asserts
        });

        self.end_frame(); // ensure CS is de-asserted before we begin
    }
}

impl<SPI: SpiX, PINS: PinsNoCS<SPI>> SpiBus<SPI, PINS> {
    /// Create a [`SharedBus`] for use with multiple devices.
    pub fn shared(spi: SPI, pins: PINS) -> SharedBus<SPI, PINS> {
        SharedBus::new(Self::new(spi, pins))
    }
}

impl<SPI: SpiX, PINS> ErrorType for SpiBus<SPI, PINS> {
    type Error = ErrorKind;
}

impl<SPI: SpiX, PINS: Pins<SPI>> FullDuplex for SpiBus<SPI, PINS> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.read_input()
    }

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.write_output(word)
    }
}

// The embedded_hal::spi::SpiBus trait can only be implemented for pin tuples
// with full ownership of the SPI bus, including MOSI, MISO, and SCK pins.
impl<SPI: SpiX, PINS: PinsFull<SPI>> spi::SpiBus for SpiBus<SPI, PINS> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let mut iwrite = 0;
        let mut iread = 0;

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while iwrite < words.len() || iread < words.len() {
            if iwrite < words.len() {
                match self.write_output(EMPTY_WRITE_PAD) {
                    Ok(()) => iwrite += 1,
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
            if iread < iwrite {
                match self.read_input() {
                    Ok(data) => {
                        unsafe { *words.get_unchecked_mut(iread) = data };
                        iread += 1;
                    }
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let mut iwrite = 0;
        let mut iread = 0;

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while iwrite < words.len() || iread < words.len() {
            if iwrite < words.len() {
                let byte = unsafe { words.get_unchecked(iwrite) };
                match self.write_output(*byte) {
                    Ok(()) => iwrite += 1,
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
            if iread < iwrite {
                match self.read_input() {
                    Ok(_) => iread += 1,
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let mut iwrite = 0;
        let mut iread = 0;
        let max_len = read.len().max(write.len());

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while iwrite < max_len || iread < max_len {
            if iwrite < max_len {
                let byte = write.get(iwrite).unwrap_or(&EMPTY_WRITE_PAD);
                match self.write_output(*byte) {
                    Ok(()) => iwrite += 1,
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
            if iread < iwrite {
                match self.read_input() {
                    Ok(data) => {
                        if let Some(byte) = read.get_mut(iread) {
                            *byte = data;
                        }
                        iread += 1;
                    }
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let mut iwrite = 0;
        let mut iread = 0;

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while iwrite < words.len() || iread < words.len() {
            if iwrite < words.len() {
                let byte = unsafe { words.get_unchecked(iwrite) };
                match self.write_output(*byte) {
                    Ok(()) => iwrite += 1,
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
            if iread < iwrite {
                match self.read_input() {
                    Ok(data) => {
                        unsafe { *words.get_unchecked_mut(iread) = data };
                        iread += 1;
                    }
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wait_for_rxfifo(); // TODO anything else to do here?
        Ok(())
    }
}
