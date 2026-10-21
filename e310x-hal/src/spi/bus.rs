use embedded_hal::{
    delay::DelayNs,
    spi::{self, ErrorKind, ErrorType, Phase, Polarity},
};
use embedded_hal_nb::spi::FullDuplex;

use super::{Pins, PinsFull, PinsNoCS, SharedBus, SpiConfig, SpiExclusiveDevice, SpiX};

use e310x::{interrupt::Priority, Plic};

const EMPTY_WRITE_PAD: u8 = 0x00;

/// Select between SPI transmission or reception for bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType {
    /// Transmission
    Tx,
    /// Reception
    Rx,
    /// Both transmission and reception
    ///
    /// # Note
    /// In the methods that check if an interrupt is enabled or pending.
    /// this event type works like an **any** operator between `Tx` and `Rx` events.
    TxRx,
}

/// Watermark values limited from 0 to 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkValue {
    /// Watermark Value 0
    W0 = 0,
    /// Watermark Value 1
    W1 = 1,
    /// Watermark Value 2
    W2 = 2,
    /// Watermark Value 3
    W3 = 3,
    /// Watermark Value 4
    W4 = 4,
    /// Watermark Value 5
    W5 = 5,
    /// Watermark Value 6
    W6 = 6,
    /// Watermark Value 7
    W7 = 7,
}

impl From<WatermarkValue> for u8 {
    fn from(w: WatermarkValue) -> u8 {
        w as u8
    }
}

impl TryFrom<u8> for WatermarkValue {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WatermarkValue::W0),
            1 => Ok(WatermarkValue::W1),
            2 => Ok(WatermarkValue::W2),
            3 => Ok(WatermarkValue::W3),
            4 => Ok(WatermarkValue::W4),
            5 => Ok(WatermarkValue::W5),
            6 => Ok(WatermarkValue::W6),
            7 => Ok(WatermarkValue::W7),
            _ => Err(()),
        }
    }
}

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
    /// Enables the external interrupt source for the SPI.
    ///
    /// # Note
    /// This function enables the external interrupt source in the PLIC,
    /// but does not enable the PLIC peripheral itself. To enable the plic peripheral
    /// you must call [`Plic::enable()`](riscv_peripheral::plic::enables::ENABLES::enable).
    ///
    /// # Safety
    /// Enabling an interrupt source can break mask-based critical sections.
    pub unsafe fn enable_exti(&mut self, plic: &Plic) {
        let ctx = plic.ctx0();
        ctx.enables().enable(SPI::INTERRUPT_SOURCE);
    }

    /// Disables the external interrupt source for the pin.
    pub fn disable_exti(&mut self, plic: &Plic) {
        let ctx = plic.ctx0();
        ctx.enables().disable(SPI::INTERRUPT_SOURCE);
    }

    /// Returns whether the external interrupt source for the pin is enabled.
    pub fn is_exti_enabled(&self, plic: &Plic) -> bool {
        let ctx = plic.ctx0();
        ctx.enables().is_enabled(SPI::INTERRUPT_SOURCE)
    }

    /// Sets the external interrupt source priority.
    ///
    /// # Safety
    ///
    /// Changing the priority level can break priority-based critical sections.
    pub unsafe fn set_exti_priority(&mut self, plic: &Plic, priority: Priority) {
        let priorities = plic.priorities();
        priorities.set_priority(SPI::INTERRUPT_SOURCE, priority);
    }

    /// Returns the external interrupt source priority.
    pub fn get_exti_priority(&self, plic: &Plic) -> Priority {
        let priorities = plic.priorities();
        priorities.get_priority(SPI::INTERRUPT_SOURCE)
    }

    /// Enable the SPI specified interrupt bit in the control register.
    ///
    /// # Note
    /// This function does not enable the interrupt in the PLIC, it only sets the
    /// interrupt enable bit in the SPI peripheral. You must call
    /// [`enable_exti()`](Self::enable_exti) to enable the interrupt in
    /// the PLIC.
    pub fn enable_interrupt(&mut self, comm_type: CommType) {
        self.enable_interrupt_inner(comm_type);
    }

    /// Shared-reference implementation of [`enable_interrupt()`](Self::enable_interrupt).
    ///
    /// Used by the asynchronous driver, where concurrent read and write futures
    /// only have a shared reference to the bus.
    pub(crate) fn enable_interrupt_inner(&self, comm_type: CommType) {
        // Match the CommType enum
        match comm_type {
            CommType::Tx => {
                // Enable Tx interrupt while keeping Rx interrupt
                self.spi.ie().modify(|r, w| {
                    w.rxwm().bit(r.rxwm().bit_is_set());
                    w.txwm().set_bit()
                });
            }
            CommType::Rx => {
                // Enable Rx interrupt while keeping Tx interrupt
                self.spi.ie().modify(|r, w| {
                    w.txwm().bit(r.txwm().bit_is_set());
                    w.rxwm().set_bit()
                });
            }
            CommType::TxRx => {
                self.spi.ie().write(|w| w.txwm().set_bit().rxwm().set_bit());
            }
        }
    }

    /// Disable the SPI interrupt enable bit in the control register.
    pub fn disable_interrupt(&mut self, comm_type: CommType) {
        self.disable_interrupt_inner(comm_type);
    }

    /// Shared-reference implementation of [`disable_interrupt()`](Self::disable_interrupt).
    ///
    /// Used by the asynchronous driver, where concurrent read and write futures
    /// only have a shared reference to the bus.
    pub(crate) fn disable_interrupt_inner(&self, comm_type: CommType) {
        // Match the CommType enum
        match comm_type {
            CommType::Tx => {
                // Disable Tx interrupt while keeping Rx interrupt
                self.spi.ie().modify(|r, w| {
                    w.rxwm().bit(r.rxwm().bit_is_set());
                    w.txwm().clear_bit()
                });
            }
            CommType::Rx => {
                // Disable Rx interrupt while keeping Tx interrupt
                self.spi.ie().modify(|r, w| {
                    w.txwm().bit(r.txwm().bit_is_set());
                    w.rxwm().clear_bit()
                });
            }
            CommType::TxRx => {
                self.spi
                    .ie()
                    .write(|w| w.txwm().clear_bit().rxwm().clear_bit());
            }
        }
    }

    /// Change the Watermark Register for the specified SPI communication to the specified value.
    pub fn set_watermark(&mut self, comm_type: CommType, watermark: WatermarkValue) {
        self.set_watermark_inner(comm_type, watermark);
    }

    /// Shared-reference implementation of [`set_watermark()`](Self::set_watermark).
    ///
    /// Used by the asynchronous driver, where concurrent read and write futures
    /// only have a shared reference to the bus.
    pub(crate) fn set_watermark_inner(&self, comm_type: CommType, watermark: WatermarkValue) {
        // Match the CommType enum
        match comm_type {
            CommType::Tx => {
                self.spi
                    .txmark()
                    .write(|w| unsafe { w.txmark().bits(watermark.into()) });
            }
            CommType::Rx => {
                self.spi
                    .rxmark()
                    .write(|w| unsafe { w.rxmark().bits(watermark.into()) });
            }
            CommType::TxRx => {
                self.spi
                    .txmark()
                    .write(|w| unsafe { w.txmark().bits(watermark.into()) });
                self.spi
                    .rxmark()
                    .write(|w| unsafe { w.rxmark().bits(watermark.into()) });
            }
        }
    }

    /// Get the Watermark Registers value.
    pub fn get_watermarks(&self) -> (u8, u8) {
        (
            self.spi.txmark().read().txmark().bits(),
            self.spi.rxmark().read().rxmark().bits(),
        )
    }

    /// Check if the SPI interrupt is enabled for the specified communication type.
    pub fn is_interrupt_enabled(&self, comm_type: CommType) -> bool {
        // Match the CommType enum
        match comm_type {
            CommType::Tx => self.spi.ie().read().txwm().bit_is_set(),
            CommType::Rx => self.spi.ie().read().rxwm().bit_is_set(),
            CommType::TxRx => {
                self.spi.ie().read().txwm().bit_is_set() || self.spi.ie().read().rxwm().bit_is_set()
            }
        }
    }

    /// Returns true if the interrupt flag is set for the specified communication type.
    pub fn is_interrupt_pending(&self, comm_type: CommType) -> bool {
        // Match the CommType enum
        match comm_type {
            CommType::Tx => self.spi.ip().read().txwm().bit_is_set(),
            CommType::Rx => self.spi.ip().read().rxwm().bit_is_set(),
            CommType::TxRx => {
                self.spi.ip().read().txwm().bit_is_set() || self.spi.ip().read().rxwm().bit_is_set()
            }
        }
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
    pub(crate) fn read_input(&self) -> nb::Result<u8, ErrorKind> {
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
    pub(crate) fn write_output(&self, word: u8) -> nb::Result<(), ErrorKind> {
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
    pub(crate) fn wait_for_rxfifo(&self) {
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
    pub fn new_device<D: DelayNs>(
        self,
        config: &SpiConfig,
        delay: D,
    ) -> SpiExclusiveDevice<SPI, PINS, D> {
        SpiExclusiveDevice::new(self, config, delay)
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
