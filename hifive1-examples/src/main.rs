//! Prints "hello world!" to the host console using the on-board UART.
//!
//! # Note
//!
//! We have noticed that using the UART while debugging with GDB can cause
//! the GDB session to hang. Thus, you will probably want to run this example
//! without GDB. Otherwise, you might not be able to see the output.

#![no_std]
#![no_main]

use hifive1::{
    hal::{prelude::*, DeviceResources},
    pin, sprintln,
};

#[cfg(not(feature = "semihosting"))]
extern crate panic_halt;
#[cfg(feature = "semihosting")]
extern crate semihosting;

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    sprintln!("Hello, world!");

    loop {}
}