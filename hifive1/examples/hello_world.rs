//! Prints "hello world!" to the host console using semihosting and the on-board UART.

#![no_std]
#![no_main]

use hifive1::{
    hal::{prelude::*, DeviceResources},
    pin, sprintln,
};
use semihosting::{println, process::exit};

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

    sprintln!("STDOUT: hello world!");
    println!("Semihosting: hello world!");

    exit(0);
}
