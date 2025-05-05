//! Basic blinking LEDs example using mtime/mtimecmp registers for "sleep" in a loop.
//! Blinks each led once and goes to the next one.
//! This example uses synchronous UART and only tests asynchronous Delay.

#![no_std]
#![no_main]

use embassy_executor::Executor;
use hifive1::{
    clock,
    hal::{e310x::CLINT, prelude::*, DeviceResources},
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

    Executor::new().run(|spawner| async {
        // get blue LED pin
        let pin = pin!(pins, led_blue);
        let mut led = pin.into_inverted_output();
        // Get the sleep struct from CLINT
        let mut sleep = CLINT::async_delay();
        const STEP: u32 = 1000; // 1s
        loop {
            Led::toggle(&mut led);
            sprintln!("LED toggled. New state: {}", led.is_on());
            sleep.delay_ms(STEP).await;
        }
    });
}
