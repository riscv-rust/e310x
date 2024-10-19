//! Prints "hello world!" to the host console.
//!
//! If "semihosting" feature is enabled, the message is printed using semihosting.
//! Otherwise, the message is printed using the UART0 peripheral.

#![no_std]
#![no_main]

extern crate panic_halt;
use hifive1::{
    hal::{prelude::*, DeviceResources},
    pin, sprintln as println,
};

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

    println!("Hello, world!");
    loop {}
}
