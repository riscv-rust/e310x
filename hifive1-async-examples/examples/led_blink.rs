//! Basic blinking LEDs example using mtime/mtimecmp registers for "sleep" in a loop.
//! Blinks each led once and goes to the next one.
//! This example uses synchronous UART and only tests asynchronous Delay.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use hifive1::{
    Led, clock,
    hal::{
        DeviceResources,
        asynch::delay::Delay,
        asynch::prelude::*,
        e310x::{Clint, interrupt::Hart},
        prelude::*,
    },
    pin, sprintln,
};
extern crate panic_halt;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
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

    // Get Mtimer
    let mtimer = unsafe { Clint::steal() }.mtimer();

    // Configure MTIMER interrupt
    mtimer.disable();
    let (mtimecmp, mtime) = (mtimer.mtimecmp(Hart::H0), mtimer.mtime());
    mtime.write(0);
    mtimecmp.write(u64::MAX);
    unsafe { riscv::interrupt::enable() };

    // Execute loop
    const STEP: u32 = 1000; // 1s
    let mut delay = Delay::new(mtimer);
    loop {
        Led::toggle(&mut led);
        let led_state = led.is_on();
        sprintln!("LED toggled. New state: {}", led_state);
        delay.delay_ms(STEP).await;
    }
}
