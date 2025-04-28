//! # Delays

use crate::clock::Clocks;
use e310x::CLINT;
use embedded_hal_async::delay::DelayNs;
use riscv::register::mip;