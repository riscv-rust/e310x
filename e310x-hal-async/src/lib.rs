//! Asynchronous HAL for the E310x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal-async`] traits for the E310x
//! family of microcontrollers.

#![deny(missing_docs)]
#![no_std]

//Temporal macro to allow for async_delays
use e310x::interrupt;
riscv_peripheral :: clint_codegen ! (base 0x2000000 , freq 32768 , mtimecmps [mtimecmp0 = (crate :: interrupt :: Hart :: H0 , "[0](crate::interrupt::Hart::H0)")] , msips [msip0 = (crate :: interrupt :: Hart :: H0 , "[0](crate::interrupt::Hart::H0)")] , async_delay ,);

pub mod delay;
