//! Basic blinking LED example using mtime/mtimecmp registers for "sleep" in a loop.
//! Blinks the blue LED of RED-V board.

#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{e310x::Clint, prelude::*, DeviceResources},
    pin, sprintln, stdout, Led,
};
extern crate panic_halt;

#[riscv_rt::core_interrupt(CoreInterrupt::MachineTimer)]
fn mtimer_handler() {
    let clint = unsafe { Clint::steal() };
    let mtimer = clint.mtimer();
    mtimer
        .mtimecmp0()
        .modify(|prev| *prev += mtimer.mtime_freq() as u64);
}

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

    // get blue LED pin
    let pin = pin!(pins, led_blue);
    let mut led = pin.into_inverted_output();

    // Get the MTIMER peripheral from CLINT
    let mtimer = cp.clint.mtimer();
    mtimer.mtime().write(0);
    mtimer.mtimecmp0().write(mtimer.mtime_freq() as u64); // Set the first compare value

    unsafe {
        mtimer.enable();
        riscv::interrupt::enable();
    }

    loop {
        Led::toggle(&mut led);
        sprintln!("LED toggled. New state: {}", led.is_on());
        riscv::asm::wfi();
    }
}
