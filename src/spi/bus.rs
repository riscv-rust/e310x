use embedded_hal::spi::blocking::Operation;
pub use embedded_hal::spi::blocking::{Read, Transfer, TransferInplace, Write, WriteIter};
pub use embedded_hal::spi::nb::FullDuplex;
pub use embedded_hal::spi::{ErrorKind, Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};

use nb;

use super::{Pins, PinsNoCS, SharedBus, SpiConfig, SpiExclusiveDevice, SpiX};

/// SPI bus abstraction
pub struct SpiBus<SPI, PINS> {
    pub(crate) spi: SPI,
    pub(crate) pins: PINS,
}

impl<SPI, PINS> SpiBus<SPI, PINS>
where
    SPI: SpiX,
{
    /// Construct the [SpiBus] for use with [SpiSharedDevice](super::SpiSharedDevice) or [SpiExclusiveDevice]
    pub fn new(spi: SPI, pins: PINS) -> Self
    where
        PINS: Pins<SPI>,
    {
        Self { spi, pins }
    }

    /// Releases the SPI peripheral and associated pins
    pub fn release(self) -> (SPI, PINS) {
        (self.spi, self.pins)
    }

    /// Configure the [SpiBus] with given [SpiConfig]
    pub(crate) fn configure(&mut self, config: &SpiConfig, cs_index: Option<u32>)
    where
        PINS: Pins<SPI>,
    {
        self.spi
            .sckdiv
            .write(|w| unsafe { w.div().bits(config.clock_divisor as u16) });

        if let Some(index) = cs_index {
            self.spi.csid.write(|w| unsafe { w.bits(index) });
        }
        self.spi.csmode.write(|w| w.mode().variant(config.cs_mode));

        // Set CS pin polarity to high
        self.spi.csdef.reset();

        // Set SPI mode
        let phase = config.mode.phase == Phase::CaptureOnSecondTransition;
        let polarity = config.mode.polarity == Polarity::IdleHigh;
        self.spi
            .sckmode
            .write(|w| w.pha().bit(phase).pol().bit(polarity));

        self.spi.fmt.write(|w| unsafe {
            w.proto().single();
            w.endian().big(); // Transmit most-significant bit (MSB) first
            w.dir().rx();
            w.len().bits(8)
        });

        // Set watermark levels
        self.spi
            .txmark
            .write(|w| unsafe { w.txmark().bits(config.txmark) });
        self.spi
            .rxmark
            .write(|w| unsafe { w.rxmark().bits(config.rxmark) });

        // set delays
        self.spi.delay0.write(|w| unsafe {
            w.cssck().bits(config.delays.cssck); // delay between assert and clock
            w.sckcs().bits(config.delays.sckcs) // delay between clock and de-assert
        });
        self.spi.delay1.write(|w| unsafe {
            w.intercs().bits(config.delays.intercs); // delay between CS re-assets
            w.interxfr().bits(config.delays.interxfr) // intra-frame delay without CS re-asserts
        });

        self.end_frame(); // ensure CS is de-asserted before we begin
    }

    fn wait_for_rxfifo(&self) {
        // Ensure that RX FIFO is empty
        while self.spi.rxdata.read().empty().bit_is_clear() {}
    }

    /// Starts frame by flagging CS assert, unless CSMODE = OFF
    pub(crate) fn start_frame(&mut self) {
        if !self.spi.csmode.read().mode().is_off() {
            self.spi.csmode.write(|w| w.mode().hold());
        }
    }

    /// Finishes frame flagging CS deassert, unless CSMODE = OFF
    pub(crate) fn end_frame(&mut self) {
        if !self.spi.csmode.read().mode().is_off() {
            self.spi.csmode.write(|w| w.mode().auto());
        }
    }

    // ex-traits now only accessible via devices

    pub(crate) fn read(&mut self) -> nb::Result<u8, ErrorKind> {
        let rxdata = self.spi.rxdata.read();

        if rxdata.empty().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits())
        }
    }

    pub(crate) fn send(&mut self, byte: u8) -> nb::Result<(), ErrorKind> {
        let txdata = self.spi.txdata.read();

        if txdata.full().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            self.spi.txdata.write(|w| unsafe { w.data().bits(byte) });
            Ok(())
        }
    }

    pub(crate) fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), ErrorKind> {
        let mut iwrite = 0;
        let mut iread = 0;

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while iwrite < write.len() || iread < read.len() {
            if iwrite < write.len() && self.spi.txdata.read().full().bit_is_clear() {
                let byte = write.get(iwrite).unwrap_or(&0);
                iwrite += 1;
                self.spi.txdata.write(|w| unsafe { w.data().bits(*byte) });
            }

            if iread < iwrite {
                let data = self.spi.rxdata.read();
                if data.empty().bit_is_clear() {
                    if let Some(d) = read.get_mut(iread) {
                        *d = data.data().bits()
                    };
                    iread += 1;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn transfer_inplace(&mut self, words: &mut [u8]) -> Result<(), ErrorKind> {
        let mut iwrite = 0;
        let mut iread = 0;

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while iwrite < words.len() || iread < words.len() {
            if iwrite < words.len() && self.spi.txdata.read().full().bit_is_clear() {
                let byte = unsafe { words.get_unchecked(iwrite) };
                iwrite += 1;
                self.spi.txdata.write(|w| unsafe { w.data().bits(*byte) });
            }

            if iread < iwrite {
                let data = self.spi.rxdata.read();
                if data.empty().bit_is_clear() {
                    unsafe { *words.get_unchecked_mut(iread) = data.data().bits() };
                    iread += 1;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn write_iter<WI>(&mut self, words: WI) -> Result<(), ErrorKind>
    where
        WI: IntoIterator<Item = u8>,
    {
        let mut iter = words.into_iter();

        let mut read_count = 0;
        let mut has_data = true;

        // Ensure that RX FIFO is empty
        self.wait_for_rxfifo();

        while has_data || read_count > 0 {
            if has_data && self.spi.txdata.read().full().bit_is_clear() {
                if let Some(byte) = iter.next() {
                    self.spi.txdata.write(|w| unsafe { w.data().bits(byte) });
                    read_count += 1;
                } else {
                    has_data = false;
                }
            }

            if read_count > 0 {
                // Read and discard byte, if any
                if self.spi.rxdata.read().empty().bit_is_clear() {
                    read_count -= 1;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn exec<'op>(
        &mut self,
        operations: &mut [Operation<'op, u8>],
    ) -> Result<(), ErrorKind> {
        for op in operations {
            match op {
                Operation::Read(words) => {
                    self.transfer(words, &[])?;
                }
                Operation::Write(words) => {
                    self.transfer(&mut [], words)?;
                }
                Operation::Transfer(read_words, write_words) => {
                    self.transfer(read_words, write_words)?;
                }
                Operation::TransferInplace(words) => {
                    self.transfer_inplace(words)?;
                }
            }
        }

        Ok(())
    }
}

impl<SPI, PINS> SpiBus<SPI, PINS>
where
    SPI: SpiX,
    PINS: Pins<SPI>,
{
    /// Create a new [SpiExclusiveDevice] for exclusive use on this bus
    pub fn new_device(self, config: &SpiConfig) -> SpiExclusiveDevice<SPI, PINS> {
        SpiExclusiveDevice::new(self, config)
    }
}

impl<SPI, PINS> SpiBus<SPI, PINS>
where
    SPI: SpiX,
    PINS: PinsNoCS<SPI>,
{
    /// Create a [SharedBus] for use with multiple devices.
    pub fn shared(spi: SPI, pins: PINS) -> SharedBus<SPI, PINS> {
        SharedBus::new(Self::new(spi, pins))
    }
}
