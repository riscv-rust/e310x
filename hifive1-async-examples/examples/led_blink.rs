//! Basic blinking LEDs example using mtime/mtimecmp registers for delay in a loop.
//! The blinking LED must be connected to pin 10 of the board
//! This example uses synchronous UART and only tests asynchronous Delay.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use hifive1::{
    clock,
    hal::{
        asynch::delay::Delay, asynch::prelude::*, e310x::interrupt::Hart, prelude::*,
        DeviceResources,
    },
    sprintln,
};
extern crate panic_halt;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let dr = DeviceResources::take().unwrap();
    let cp = dr.core_peripherals;
    let p = dr.peripherals;
    let pins: hifive1::hal::device::DeviceGpioPins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    //Blinking LED
    let mut led = pins.pin10.into_output();

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pins.pin17, pins.pin16, 115_200.bps(), clocks);

    // Get Mtimer
    let mtimer = cp.clint.mtimer();

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
        led.toggle().unwrap();
        let led_state = led.is_set_high().unwrap();
        sprintln!("LED toggled. New state: {}", led_state);
        delay.delay_ms(STEP).await;
    }
}
