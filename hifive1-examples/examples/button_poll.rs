//! Example of polling a button and turning on an LED when the button is pressed.
//!
//! # Hardware
//!
//! - HiFive1 or RED-V board
//! - A button connected to pin 9

#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{prelude::*, DeviceResources},
    pin, sprintln, stdout, Led,
};
extern crate panic_halt;

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let cp = dr.core_peripherals;
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    // Configure button pin as pull-up input
    let mut button = pins.pin9.into_pull_up_input();

    // get blue LED pin
    let pin = pin!(pins, led_blue);
    let mut led = pin.into_inverted_output();

    // Get the MTIMER peripheral from CLINT
    let mut mtimer = cp.clint.mtimer();

    const STEP: u32 = 1000; // 1s
    loop {
        if button.is_low().unwrap() {
            sprintln!("Button pressed");
            led.on();
        } else {
            sprintln!("Button released");
            led.off();
        }
        sprintln!("LED is on: {}", led.is_on());
        mtimer.delay_ms(STEP);
    }
}
