//! Demonstration of the PWM peripheral to control the brightness of an LED.
//!
//! # Hardware
//!
//! - HiFive1 or Red-V board
//! - LED connected to pin 1
#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{e310x::CLINT, prelude::*, DeviceResources},
    pin, sprintln,
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

    // get blue LED pin
    let pin = pin!(pins, pwm0_cmp1).into_inverted_iof1();

    let mut pwm0 = hifive1::hal::pwm::Pwm::new(p.PWM0);
    pwm0.set_period(255);

    let mut channel = pwm0.channel(pin);

    // Get the sleep struct from CLINT
    let mut sleep = CLINT::delay();

    const STEP: u32 = 1000; // 1s
    const DUTY_DELTA: u8 = 32;

    let mut duty: u8 = 0;
    loop {
        sprintln!("Duty: {}", duty);
        channel.set_duty_cycle(duty as u16).unwrap();
        duty = duty.wrapping_add(DUTY_DELTA);

        sleep.delay_ms(STEP);
    }
}
