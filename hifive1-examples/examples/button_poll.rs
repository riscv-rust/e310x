//! Example of polling a button and turning on a LED when the button is pressed.

#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{delay::Sleep, prelude::*, DeviceResources},
    pin, sprintln, Led,
};
extern crate panic_halt;

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(
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

    // Get the sleep struct from CLINT
    let clint = dr.core_peripherals.clint;
    let mut sleep = Sleep::new(clint.mtimecmp, clocks);

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
        sleep.delay_ms(STEP);
    }
}
