//! Optional external interrupt handler
use crate::core::CorePeripherals;

/* Empty definition of all external handler functions, to be provided
   in the .x file and declared by the user */
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
    fn I2C0();
}

#[no_mangle]
/// Array of handlers
pub static HANDLERS: [unsafe extern "C" fn(); 52] = [
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
    I2C0,
];

#[no_mangle]
/// Default external handler
pub fn DefaultMachineExternal() {
    loop {
        /* do nothing, but we need a side effect so LLVM does not optimize
        this */
        continue;
    }
}
/// Optional handler to use for external interrupts.
/// It will automatically claim any interrupt and call the appropriate
/// handler function (ex.: GPIO0() for GPIO0 interrupts).
/// The handler functions can be overriden by the user, otherwise default
/// behavior will be called.
#[no_mangle]
//#[feature(virq)]
unsafe fn MachineExternal() {
    /* Steal the PLIC peripheral to claim the interrupt */
    let mut plic = CorePeripherals::steal().plic;
    let interrupt = plic.claim.claim().unwrap();
    /* Match the appropriate external interrupt */
    /* Interrupt 0 is defined as no interrupt, so we treat it independently */
    if interrupt as u16 == 0 { DefaultMachineExternal() }
    else {
        // Offset the handlers as we do not work with interrupt = 0
        HANDLERS[(interrupt as usize) - 1]();
    }
    plic.claim.complete(interrupt);
}
