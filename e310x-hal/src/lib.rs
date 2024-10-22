//! HAL for the E310x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the E310x
//! family of microcontrollers.
//!
//! # Building an application for the E310x chips
//!
//! The E310x chips implement the [Zaamo](https://github.com/riscv/riscv-zaamo-zalrsc/blob/main/zaamo-zalrsc.adoc)
//! extension for atomic instructions. This means that it *partially* supports the `A`
//! extension for atomic. Specifically, it supports the `amo*` instructions, but not the
//! `lr*` and `sc*` instructions.
//!
//! It is discouraged to use the `riscv32imac-unknown-none-elf` target for E310x chips, as it
//! will potentially generate code that uses the `lr*` and `sc*` instructions, which are not
//! supported by the E310x chips. Thus, it is recommended to use `riscv32imc-unknown-none-elf`.
//!
//! # Working with atomic operations
//!
//! If you are using the `riscv32imc-unknown-none-elf` target, you will notice that
//! `core::sync::atomic` is not available. To work around this, you can use the
//! [`portable-atomic`](https://docs.rs/portable-atomic/1.8.0/portable_atomic/) crate.
//! This crate allows us to use native `amo*` instructions on the E310x chips without requiring
//! the `A` extension. Furthermore, you can emulate the `lr*` and `sc*` instructions if needed.
//!
//! Thus, the recommended way to work with E310x chips is:
//!
//! 1. Compile your code against the `riscv32imc-unknown-none-elf` target.
//! 2. Add the following configuration to your `.cargo/config.toml`:
//!
//! ```toml
//! [target.'cfg(all(target_arch = "riscv32", target_os = "none"))']
//! rustflags = [
//!    "--cfg", "portable_atomic_target_feature=\"zaamo\"",
//! ]
//!
//! [build]
//! target = "riscv32imc-unknown-none-elf"
//! ```
//!
//! This will ensure that the `portable-atomic` crate is correctly configured to work with the E310x chips.

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
