//! Demonstration on how to configure the GPIO9 interrupt on HiFive boards.
#![no_main]
#![no_std]

use core::cell::RefCell;
use critical_section::Mutex;
use hifive1::{
    clock,
    hal::{
        gpio::{gpio0, EventType, Input, PullUp},
        prelude::*,
        DeviceResources,
    },
    pin, sprintln, stdout, Led,
};
extern crate panic_halt;

static BUTTON: Mutex<RefCell<Option<gpio0::Pin9<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

/* Handler for the GPIO9 interrupt */
#[riscv_rt::external_interrupt(ExternalInterrupt::GPIO9)]
fn gpio9_handler() {
    sprintln!("GPIO9 interrupt!");
    // Take the button
    critical_section::with(|cs| {
        let mut button_ref = BUTTON.borrow_ref_mut(cs);
        let button = button_ref.as_mut().unwrap();

        // Check the interrupt source
        if button.is_interrupt_pending(EventType::Rise) {
            sprintln!("Rising Edge");
        }
        if button.is_interrupt_pending(EventType::Fall) {
            sprintln!("Falling Edge");
        }

        // Clear the interrupt
        button.clear_interrupt(EventType::BothEdges);
    });
}

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let cp = dr.core_peripherals;
    let p = dr.peripherals;
    let mut pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Disable and clear all GPIO interrupts
    pins.disable_interrupts(EventType::All);
    pins.clear_interrupts(EventType::All);

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
    unsafe { button.set_exti_priority(&plic, Priority::P1) };
    button.enable_interrupt(EventType::BothEdges);

    // Store button pin in a shared resource
    critical_section::with(|cs| {
        BUTTON.borrow(cs).replace(Some(button));
    });

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
        // Check if the button is low
        let mut button_state = false;
        critical_section::with(|cs| {
            button_state = BUTTON
                .borrow_ref_mut(cs)
                .as_mut()
                .unwrap()
                .is_low()
                .unwrap();
        });

        if button_state {
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
