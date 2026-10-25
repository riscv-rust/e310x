//! # SPI Asynchronous Implementation
//! # Note
//!
//! Implementation of the Async Embedded HAL SPI functionality.
//!

mod bus; // contains the SPI Bus abstraction
mod exclusive_device; // contains the exclusive SPI device abstraction
mod shared_bus; // contains the shared SPI bus abstraction
mod shared_device; // contains the shared SPI device abstraction

pub use exclusive_device::*;
pub use shared_bus::*;
pub use shared_device::*;
