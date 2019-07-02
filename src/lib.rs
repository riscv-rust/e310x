//! HAL for the E310x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the E310x
//! family of microcontrollers.

#![deny(missing_docs)]
#![no_std]

pub use e310x;

pub mod core;
pub mod clock;
pub mod delay;
pub mod gpio;
pub mod device;
pub mod plic;
pub mod prelude;
pub mod rtc;
pub mod serial;
pub mod spi;
pub mod stdout;
pub mod time;
pub mod wdog;

#[cfg(feature = "g002")]
pub mod i2c;

pub use device::DeviceResources;
