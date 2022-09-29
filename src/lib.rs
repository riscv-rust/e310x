//! HAL for the E310x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the E310x
//! family of microcontrollers.

#![deny(missing_docs)]
#![no_std]

pub use e310x;

pub mod clock;
pub mod core;
pub mod delay;
pub mod device;
pub mod gpio;
pub mod pmu;
pub mod prelude;
pub mod pwm;
pub mod rtc;
pub mod serial;
pub mod spi;
pub mod stdout;
pub mod time;
pub mod wdog;

#[cfg(feature = "g002")]
pub mod i2c;

pub use device::DeviceResources;
