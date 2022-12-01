//! Optional external interrupt handler
use crate::core::CorePeripherals;

/* Empty definition of all external handler functions */
extern "C" {
    /// w
    fn GPIO4();
}

#[no_mangle]
/// Array of handlers
pub static HANDLERS: [unsafe extern "C" fn(); 1] = [GPIO4];

#[no_mangle]
/// Default external handler
pub fn DefaultMachineExternal() {
    loop {
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
    let mut plic = CorePeripherals::steal().plic;
    let interrupt = plic.claim.claim().unwrap();
    /* Match the appropriate external interrupt */
    match interrupt {
        e310x::Interrupt::GPIO4 => HANDLERS[0](),
        _ => {}
    }
    plic.claim.complete(interrupt);
}
