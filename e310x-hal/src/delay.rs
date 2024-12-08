//! # Delays

use crate::clock::Clocks;
use e310x::CLINT;
use embedded_hal::delay::DelayNs;
use riscv::register::mip;

/// Machine timer (mtime) as a busyloop delay provider
#[derive(Default)]
pub struct Delay;

const TICKS_PER_SECOND: u64 = 32768;

impl Delay {
    /// Constructs a delay provider based on the machine timer (mtime)
    pub fn new() -> Self {
        Delay
    }
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = (ns as u64) * TICKS_PER_SECOND / 1_000_000_000;

        let mtime = CLINT::mtimer().mtime;
        let t = mtime.read() + ticks;
        while mtime.read() < t {}
    }
}

/// Machine timer (mtime) as a sleep delay provider using mtimecmp
pub struct Sleep {
    clock_freq: u32,
}

impl Sleep {
    /// Constructs a delay provider using mtimecmp register to sleep
    pub fn new(clocks: Clocks) -> Self {
        Sleep {
            clock_freq: clocks.lfclk().0,
        }
    }
}

impl DelayNs for Sleep {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = (ns as u64) * u64::from(self.clock_freq) / 1_000_000_000;
        let t = CLINT::mtimer().mtime.read() + ticks;

        CLINT::mtimecmp0().write(t);

        // Enable timer interrupt
        unsafe { CLINT::mtimer_enable() };

        // Wait For Interrupt will put CPU to sleep until an interrupt hits
        // in our case when internal timer mtime value >= mtimecmp value
        // after which empty handler gets called and we go into the
        // next iteration of this loop
        loop {
            riscv::asm::wfi();

            // check if we got the right interrupt cause, otherwise just loop back to wfi
            if mip::read().mtimer() {
                break;
            }
        }

        // Clear timer interrupt
        CLINT::mtimer_disable();
    }
}
