//! # Serial Peripheral Interface
//!
//! You can use the `Spi` interface with these SPI instances
//!
//! # QSPI0
//! - Interrupt::QSPI0
//!
//! # QSPI1
//! - MOSI: Pin 3 IOF0
//! - MISO: Pin 4 IOF0
//! - SCK: Pin 5 IOF0
//! - CS0: Pin 2 IOF0
//! - CS1: Pin 8 IOF0 (not connected to package in FE310)
//! - CS2: Pin 9 IOF0
//! - CS3: Pin 10 IOF0
//! - Interrupt::QSPI1
//!
//! # QSPI2
//! *Warning:* QSPI2 pins are not connected to package in FE310
//! - MOSI: Pin 27 IOF0
//! - MISO: Pin 28 IOF0
//! - SCK: Pin 29 IOF0
//! - CS: Pin 26 IOF0
//! - Interrupt::QSPI2
//!
//! # Exclusive Bus usage example
//!```ignore
//! let pins = (mosi, miso, sck, cs0);
//! let spi_bus = SpiBus::new(p.QSPI1, pins);
//!
//! let spi_config = SpiConfig::new(MODE_0, 100.khz().into(), &clocks);
//! let mut dev = spi_bus.new_device(&spi_config);
//!
//! dev.write(&[1, 2, 3]).unwrap();
//!```
//!
//! # Shared Bus usage example
//!```ignore
//! let pins = (mosi, miso, sck);
//! let spi_bus = SpiBus::shared(p.QSPI1, pins);
//!
//! let spi_config1 = SpiConfig::new(MODE_0, 100.khz().into(), &clocks);
//! let mut dev1 = spi_bus.new_device(cs0, &spi_config1);
//!
//! let spi_config2 = SpiConfig::new(MODE_3, 2.mhz().into(), &clocks);
//! let mut dev2 = spi_bus.new_device(cs1, &spi_config2);
//!
//! dev1.write(&[1, 2, 3]).unwrap();
//! dev2.write(&[4, 5]).unwrap();
//!```

mod bus; // contains the SPI Bus abstraction
mod config;
mod exclusive_device; // contains the exclusive SPI device abstraction
mod shared_bus; // shared bus newtype
mod shared_device; // contains the shared SPI device abstraction
mod traits; // contains SPI device abstraction

pub use bus::*;
pub use config::*;
pub use exclusive_device::*;
pub use shared_bus::*;
pub use shared_device::*;
pub use traits::*;
