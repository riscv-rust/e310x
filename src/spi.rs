//! # Serial Peripheral Interface
//!
//! You can use the `FullDuplex` interface with these SPI instances
//!
//! # QSPI0
//! - Interrupt::QSPI0
//!
//! # QSPI1
//! - MOSI: Pin 3 IOF0
//! - MISO: Pin 4 IOF0
//! - SCK: Pin 5 IOF0
//! - SS0: Pin 2 IOF0
//! - SS1: Pin 8 IOF0
//! - SS2: Pin 9 IOF0
//! - SS3: Pin 10 IOF0
//! - Interrupt::QSPI1
//!
//! # QSPI2
//! *Warning:* QSPI2 pins are not connected to package in FE310-G000
//! - MOSI: Pin 27 IOF0
//! - MISO: Pin 28 IOF0
//! - SCK: Pin 29 IOF0
//! - SS: Pin 26 IOF0
//! - Interrupt::QSPI0

pub use embedded_hal::spi::{Mode, Phase, Polarity};
use crate::e310x::{QSPI0, QSPI1, QSPI2};
use crate::gpio::{IOF0, gpio0};
use crate::clock::Clocks;
use crate::time::Hertz;
use nb;
use void::Void;


/// SPI pins - DO NOT IMPLEMENT THIS TRAIT
///
/// This trait is implemented for pin tuples (), (MOSI, MISO, SCK) and (MOSI, MISO, SCK, SS)
pub trait Pins<SPI> {
    #[doc(hidden)]
    const CS_INDEX: Option<u32>;
}

/* SPI0 pins */
impl Pins<QSPI0> for () {
    const CS_INDEX: Option<u32> = Some(0);
}

/* SPI1 pins */
impl<T> Pins<QSPI1> for (gpio0::Pin3<IOF0<T>>, gpio0::Pin4<IOF0<T>>, gpio0::Pin5<IOF0<T>>) {
    const CS_INDEX: Option<u32> = None;
}

impl<T> Pins<QSPI1> for (gpio0::Pin3<IOF0<T>>, gpio0::Pin4<IOF0<T>>, gpio0::Pin5<IOF0<T>>, gpio0::Pin2<IOF0<T>>) {
    const CS_INDEX: Option<u32> = Some(0);
}

impl<T> Pins<QSPI1> for (gpio0::Pin3<IOF0<T>>, gpio0::Pin4<IOF0<T>>, gpio0::Pin5<IOF0<T>>, gpio0::Pin8<IOF0<T>>) {
    const CS_INDEX: Option<u32> = Some(1);
}

impl<T> Pins<QSPI1> for (gpio0::Pin3<IOF0<T>>, gpio0::Pin4<IOF0<T>>, gpio0::Pin5<IOF0<T>>, gpio0::Pin9<IOF0<T>>) {
    const CS_INDEX: Option<u32> = Some(2);
}

impl<T> Pins<QSPI1> for (gpio0::Pin3<IOF0<T>>, gpio0::Pin4<IOF0<T>>, gpio0::Pin5<IOF0<T>>, gpio0::Pin10<IOF0<T>>) {
    const CS_INDEX: Option<u32> = Some(3);
}

/* SPI2 pins */
impl<T> Pins<QSPI2> for (gpio0::Pin27<IOF0<T>>, gpio0::Pin28<IOF0<T>>, gpio0::Pin29<IOF0<T>>) {
    const CS_INDEX: Option<u32> = None;
}

impl<T> Pins<QSPI2> for (gpio0::Pin27<IOF0<T>>, gpio0::Pin28<IOF0<T>>, gpio0::Pin29<IOF0<T>>, gpio0::Pin26<IOF0<T>>) {
    const CS_INDEX: Option<u32> = Some(0);
}

/// SPI abstraction
pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

macro_rules! hal {
    ($($SPIX:ident: $spiX:ident,)+) => {
        $(
            impl <PINS> Spi<$SPIX, PINS> {
                /// Configures the SPI peripheral to operate in full duplex master mode
                pub fn $spiX(
                    spi: $SPIX,
                    pins: PINS,
                    mode: Mode,
                    freq: Hertz,
                    clocks: Clocks,
                ) -> Self
                where
                    PINS: Pins<$SPIX>,
                {
                    let div = clocks.tlclk().0 / (2 * freq.0) - 1;
                    spi.div.write(|w| unsafe { w.bits(div) });

                    let cs_mode = if let Some(cs_index) = PINS::CS_INDEX {
                        spi.csid.write(|w| unsafe { w.bits(cs_index) });

                        0 // AUTO: Assert/de-assert CS at the beginning/end of each frame
                    } else {
                        3 // OFF: Disable hardware control of the CS pin
                    };
                    spi.csmode.write(|w| unsafe { w.bits(cs_mode) });

                    // Set CS pin polarity to high
                    spi.csdef.reset();

                    // Set SPI mode
                    let phase = mode.phase == Phase::CaptureOnSecondTransition;
                    let polarity = mode.polarity == Polarity::IdleHigh;
                    spi.mode.write(|w| w
                        .phase().bit(phase)
                        .polarity().bit(polarity)
                    );

                    spi.fmt.write(|w| unsafe { w
                        .protocol().bits(0) // Single
                        .endian().clear_bit() // Transmit most-significant bit (MSB) first
                        .direction().rx()
                        .length().bits(8)
                    });

                    // Set watermark levels
                    spi.txmark.write(|w| unsafe { w.value().bits(1) });
                    spi.rxmark.write(|w| unsafe { w.value().bits(0) });

                    spi.delay0.reset();
                    spi.delay1.reset();

                    Self { spi, pins }
                }

                /// Sets transmit watermark level
                pub fn set_tx_watermark(&mut self, value: u8) {
                    self.spi.txmark.write(|w| unsafe { w.value().bits(value) });
                }

                /// Sets receive watermark level
                pub fn set_rx_watermark(&mut self, value: u8) {
                    self.spi.rxmark.write(|w| unsafe { w.value().bits(value) });
                }

                /// Returns transmit watermark event status
                pub fn tx_wm_is_pending(&self) -> bool {
                    self.spi.ip.read().txwm().bit()
                }

                /// Returns receive watermark event status
                pub fn rx_wm_is_pending(&self) -> bool {
                    self.spi.ip.read().rxwm().bit()
                }

                /// Starts listening for transmit watermark interrupt event
                pub fn listen_tx_wm(&mut self) {
                    self.spi.ie.write(|w| w.txwm().set_bit())
                }

                /// Starts listening for receive watermark interrupt event
                pub fn listen_rx_wm(&mut self) {
                    self.spi.ie.write(|w| w.rxwm().set_bit())
                }

                /// Stops listening for transmit watermark interrupt event
                pub fn unlisten_tx_wm(&mut self) {
                    self.spi.ie.write(|w| w.txwm().clear_bit())
                }

                /// Stops listening for receive watermark interrupt event
                pub fn unlisten_rx_wm(&mut self) {
                    self.spi.ie.write(|w| w.rxwm().clear_bit())
                }

                /// Releases the SPI peripheral and associated pins
                pub fn free(self) -> ($SPIX, PINS) {
                    (self.spi, self.pins)
                }
            }

            impl<PINS> embedded_hal::spi::FullDuplex<u8> for Spi<$SPIX, PINS> {
                type Error = void::Void;

                fn read(&mut self) -> nb::Result<u8, Void> {
                    let rxdata = self.spi.rxdata.read();

                    if rxdata.empty().bit_is_set() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        Ok(rxdata.data().bits())
                    }
                }

                fn send(&mut self, byte: u8) -> nb::Result<(), Void> {
                    let txdata = self.spi.txdata.read();

                    if txdata.full().bit_is_set() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.spi.txdata.write(|w| unsafe { w.data().bits(byte) });
                        Ok(())
                    }
                }
            }

            impl<PINS> embedded_hal::blocking::spi::transfer::Default<u8> for Spi<$SPIX, PINS> {}

            impl<PINS> embedded_hal::blocking::spi::write::Default<u8> for Spi<$SPIX, PINS> {}
        )+
    }
}

hal! {
    QSPI0: spi0,
    QSPI1: spi1,
    QSPI2: spi2,
}
