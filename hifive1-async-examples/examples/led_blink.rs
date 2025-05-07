//! Basic blinking LEDs example using mtime/mtimecmp registers for "sleep" in a loop.
//! Blinks each led once and goes to the next one.
//! This example uses synchronous UART and only tests asynchronous Delay.

#![no_std]
#![no_main]

use e310x_hal_async::{CLINT, prelude::*};
use embassy_executor::Executor;
use hifive1::{
    BLUE, Led, clock,
    hal::{DeviceResources, prelude::*},
    pin, sprintln,
};
use static_cell::StaticCell;
extern crate panic_halt;

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::task]
async fn blink(mut led: BLUE) {
    // Get the sleep struct from CLINT
    let mut sleep = CLINT::async_delay();
    const STEP: u32 = 1000; // 1s
    loop {
        Led::toggle(&mut led);
        sprintln!("LED toggled. New state: {}", led.is_on());
        sleep.delay_ms(STEP).await;
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins: hifive1::hal::device::DeviceGpioPins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    //Blinking LED
    let pin = pin!(pins, led_blue);
    let mut led = pin.into_inverted_output();

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    //Execute task
    Led::toggle(&mut led);
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(blink(led)).unwrap();
    });
}
