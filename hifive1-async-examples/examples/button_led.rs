//! Basic example where the LED changes its state according to a button connected to pin 9.
//! The LED must be connected to pin 10 of the board
//! This example uses synchronous UART and only tests asynchronous GPIO.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use hifive1::{
    clock,
    hal::{asynch::prelude::*, gpio::EventType, prelude::*, DeviceResources},
    sprintln,
};
extern crate panic_halt;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let dr = DeviceResources::take().unwrap();
    let cp = dr.core_peripherals;
    let p = dr.peripherals;
    let mut pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Disable and clear pending GPIO interrupts from previous states
    pins.disable_interrupts(EventType::All);
    pins.clear_interrupts(EventType::All);

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pins.pin17, pins.pin16, 115_200.bps(), clocks);

    sprintln!("Configuring GPIOs...");

    // Button pin (GPIO9) as pull-up input
    let mut button = pins.pin9.into_pull_up_input();
    // Configure blue LED pin (GPIO10) as output
    let mut led = pins.pin10.into_output();

    sprintln!("Configuring external interrupts...");

    // Make sure interrupts are disabled
    riscv::interrupt::disable();

    // Reset PLIC interrupts and set priority threshold
    let plic = cp.plic;
    let priorities = plic.priorities();
    let ctx = plic.ctx0();
    priorities.reset::<ExternalInterrupt>();
    unsafe {
        ctx.enables().disable_all::<ExternalInterrupt>();
        ctx.threshold().set_threshold(Priority::P0);
    }

    // Enable GPIO9 interrupt for both edges
    button.enable_interrupt(EventType::BothEdges);
    unsafe {
        button.set_exti_priority(&plic, Priority::P1);
        button.enable_exti(&plic);
    }

    sprintln!("Enabling external interrupts...");

    // Enable global interrupts
    unsafe {
        riscv::interrupt::enable();
        plic.enable();
    }

    // Execute loop
    loop {
        led.toggle().unwrap();
        let led_state = led.is_set_high().unwrap();
        sprintln!("LED toggled. New state: {}", led_state);
        button.wait_for_any_edge().await.unwrap();
    }
}
