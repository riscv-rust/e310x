//! # Delays

use crate::clock::Clocks;
use e310x::CLINT;
use embedded_hal::delay::DelayNs;
use riscv::register::mip;