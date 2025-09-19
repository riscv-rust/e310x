//! Demonstration on how to configure the GPIO9 interrupt on HiFive boards.
#![no_main]
#![no_std]

use hifive1::{
    clock,
    hal::DeviceResources,
    hal::{e310x::Gpio0, prelude::*},
    pin, sprintln, stdout, Led,
};
extern crate panic_halt;

/* Handler for the GPIO9 interrupt */
#[riscv_rt::external_interrupt(ExternalInterrupt::GPIO9)]
fn gpio9_handler() {
    sprintln!("GPIO9 interrupt!");
    // Clear the GPIO pending interrupt
    let gpio_block = unsafe { Gpio0::steal() };
    let _prev_fall = gpio_block.fall_ip().read().pin9().bit_is_set();
    let _prev_rise = gpio_block.rise_ip().read().pin9().bit_is_set();
    // Clear the interrupt by writing 1 to the pending bit
    gpio_block.fall_ip().write(|w| w.pin9().set_bit());
    gpio_block.rise_ip().write(|w| w.pin9().set_bit());
    let _post_fall = gpio_block.fall_ip().read().pin9().bit_is_set();
    let _post_rise = gpio_block.rise_ip().read().pin9().bit_is_set();
    let _ok = _prev_fall || _prev_rise; // Ensure we cleared the interrupt
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

    sprintln!("Configuring GPIOs...");
    // Configure button pin (GPIO9) as pull-up input
    let mut button = pins.pin9.into_pull_up_input();
    // Configure blue LED pin (GPIO21) as inverted output
    let mut led = pin!(pins, led_blue).into_inverted_output();

    sprintln!("Configuring external interrupts...");
    // Set button interrupt source priority
    let plic = cp.plic;
    let priorities = plic.priorities();
    priorities.reset::<ExternalInterrupt>();
    unsafe { priorities.set_priority(ExternalInterrupt::GPIO9, Priority::P1) };

    // Enable GPIO9 interrupt for both edges
    let gpio_block = unsafe { Gpio0::steal() };
    unsafe {
        // Clear pending interrupts from previous states
        gpio_block.low_ie().write(|w| w.bits(0x00000000));
        gpio_block.high_ie().write(|w| w.bits(0x00000000));
        gpio_block.fall_ie().write(|w| w.bits(0x00000000));
        gpio_block.rise_ie().write(|w| w.bits(0x00000000));
        gpio_block.low_ip().write(|w| w.bits(0xffffffff));
        gpio_block.high_ip().write(|w| w.bits(0xffffffff));
        gpio_block.fall_ip().write(|w| w.bits(0xffffffff));
        gpio_block.rise_ip().write(|w| w.bits(0xffffffff));
    }
    gpio_block.fall_ie().write(|w| w.pin9().set_bit());
    gpio_block.rise_ie().write(|w| w.pin9().set_bit());

    sprintln!("Enabling external interrupts...");
    // Enable GPIO9 interrupt in PLIC
    let ctx = plic.ctx0();
    unsafe {
        ctx.enables().disable_all::<ExternalInterrupt>();
        ctx.threshold().set_threshold(Priority::P0);
        ctx.enables().enable(ExternalInterrupt::GPIO9);
        riscv::interrupt::enable();
        plic.enable();
    }

    loop {
        if button.is_low().unwrap() {
            sprintln!("Button pressed");
            led.on();
        } else {
            sprintln!("Button released");
            led.off();
        }
        sprintln!("LED is on: {}", led.is_on());
        riscv::asm::wfi();
    }
}
