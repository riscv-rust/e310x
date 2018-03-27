//! HAL for the E310x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the E310x
//! family of microcontrollers.

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]
#![feature(try_from)]

extern crate embedded_hal as hal;
#[macro_use]
extern crate nb;

extern crate riscv;
pub extern crate e310x;

pub mod clint;
pub mod clock;
pub mod gpio;
pub mod plic;
pub mod prelude;
pub mod serial;
pub mod stdout;
pub mod time;
