//! Vectored machine external interrupt handler.
//!
//! # Notes
//!
//! - You must activate the `virq` feature to use this module.
//!
//! - The vectored handler automatically claims the PLIC interrupt source as complete.
//!   Thus, users do not have to worry about this step.
//!
//! # Features
//!
//! This module provides:
//!
//! - A vectored implementation for handling each machine external interrupt source independently.
//!
//! - A linker script that provides weak symbols for all the interrupt sources of an E310X microcontroller.
//!   This file must be supplied using rustflag when compiling.
//!
//! # Implementation details
//!
//! You can define a custom handler for each interrupt source (see [`e310x::interrupt::Interrupt`]).
//! For instance, if you want to define a custom handler for interrupts triggered by
//!  the [`e310x::interrupt::Interrupt::GPIO0`] source, you must define the `GPIO0` function:
//!
//! ```ignore
//! #[no_mangle]
//! #[allow(non_snake_case)]
//! fn GPIO0() {
//!     // define the behavior of your custom handler
//! }
//! ```
//!
//! Note that the function must be marked as `no_mangle`.
//! You can also use the [`e310x::interrupt!`] macro.
//!
//! If a source without custom handler triggers an interruption, it executes the
//! `OtherMachineExternal` handler. This handler function is shared among all the
//! undefined interrupt sources. You can define this handler as follows:
//!
//! ```ignore,no_run
//! #[no_mangle]
//! #[allow(non_snake_case)]
//! fn OtherMachineExternal() {
//!     // define the behavior of this handler
//! }
//! ```
//!
//! By default, `OtherMachineExternal` executes the [`DefaultMachineExternal`] handler.
//! This handler is just an infinite loop.

use crate::core::CorePeripherals;
pub use e310x::interrupt::*;

extern "C" {
    fn WATCHDOG();
    fn RTC();
    fn UART0();
    fn UART1();
    fn QSPI0();
    fn QSPI1();
    fn QSPI2();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn GPIO5();
    fn GPIO6();
    fn GPIO7();
    fn GPIO8();
    fn GPIO9();
    fn GPIO10();
    fn GPIO11();
    fn GPIO12();
    fn GPIO13();
    fn GPIO14();
    fn GPIO15();
    fn GPIO16();
    fn GPIO17();
    fn GPIO18();
    fn GPIO19();
    fn GPIO20();
    fn GPIO21();
    fn GPIO22();
    fn GPIO23();
    fn GPIO24();
    fn GPIO25();
    fn GPIO26();
    fn GPIO27();
    fn GPIO28();
    fn GPIO29();
    fn GPIO30();
    fn GPIO31();
    fn PWM0CMP0();
    fn PWM0CMP1();
    fn PWM0CMP2();
    fn PWM0CMP3();
    fn PWM1CMP0();
    fn PWM1CMP1();
    fn PWM1CMP2();
    fn PWM1CMP3();
    fn PWM2CMP0();
    fn PWM2CMP1();
    fn PWM2CMP2();
    fn PWM2CMP3();
    #[cfg(feature = "g002")]
    fn I2C0();
}

#[no_mangle]
#[allow(non_snake_case)]
/// Default machine external interrupt handler. It is an infinite loop.
pub fn DefaultMachineExternal() {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

#[cfg(not(feature = "g002"))]
const N_INTERRUPTS: usize = 51;
#[cfg(feature = "g002")]
const N_INTERRUPTS: usize = 52;

/// Array of machine external interrupt handlers.
static HANDLERS: [unsafe extern "C" fn(); N_INTERRUPTS] = [
    WATCHDOG,
    RTC,
    UART0,
    UART1,
    QSPI0,
    QSPI1,
    QSPI2,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
    GPIO10,
    GPIO11,
    GPIO12,
    GPIO13,
    GPIO14,
    GPIO15,
    GPIO16,
    GPIO17,
    GPIO18,
    GPIO19,
    GPIO20,
    GPIO21,
    GPIO22,
    GPIO23,
    GPIO24,
    GPIO25,
    GPIO26,
    GPIO27,
    GPIO28,
    GPIO29,
    GPIO30,
    GPIO31,
    PWM0CMP0,
    PWM0CMP1,
    PWM0CMP2,
    PWM0CMP3,
    PWM1CMP0,
    PWM1CMP1,
    PWM1CMP2,
    PWM1CMP3,
    PWM2CMP0,
    PWM2CMP1,
    PWM2CMP2,
    PWM2CMP3,
    #[cfg(feature = "g002")]
    I2C0,
];

/// Handler for vectored machine external interrupts (see the [`riscv-rt`] crate).
#[no_mangle]
#[allow(non_snake_case)]
unsafe fn MachineExternal() {
    // Steal the PLIC peripheral and claim the interrupt
    let mut plic = CorePeripherals::steal().plic;
    let interrupt = plic.claim.claim().unwrap();
    let interrupt_n = interrupt as usize;
    // Match the appropriate machine external interrupt
    if interrupt_n == 0 {
        // Interrupt number 0 is defined as no interrupt
    } else if interrupt_n <= HANDLERS.len() {
        // Execute corresponding interrupt handler
        HANDLERS[interrupt_n - 1]();
    } else {
        // Any other interrupt number is not allowed
        DefaultMachineExternal();
    }
    // Claim PLIC interrupt source as complete by this handler
    plic.claim.complete(interrupt);
}
