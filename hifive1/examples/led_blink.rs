//! Basic blinking LEDs example using mtime/mtimecmp registers for "sleep" in a loop.
//! Blinks each led once and goes to the next one.

#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{delay::Sleep, prelude::*, DeviceResources},
    pin, Led,
};
use semihosting::println;

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // get all 3 led pins in a tuple (each pin is it's own type here)
    let pin = pin!(pins, led_blue);
    let mut led = pin.into_inverted_output();

    // Get the sleep struct from CLINT
    let clint = dr.core_peripherals.clint;
    let mut sleep = Sleep::new(clint.mtimecmp, clocks);

    println!("Starting blink loop");

    const STEP: u32 = 1000; // 1s
    loop {
        Led::toggle(&mut led);
        println!("LED toggled. New state: {}", led.is_on());
        sleep.delay_ms(STEP);
    }
}
