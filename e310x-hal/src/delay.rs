//! # Delays

use crate::clock::Clocks;
use crate::core::clint::{MTIME, MTIMECMP};
use embedded_hal::delay::DelayNs;
use riscv::register::{mie, mip};

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

        let mtime = MTIME;
        let t = mtime.mtime() + ticks;
        while mtime.mtime() < t {}
    }
}

/// Machine timer (mtime) as a sleep delay provider using mtimecmp
pub struct Sleep {
    clock_freq: u32,
    mtimecmp: MTIMECMP,
}

impl Sleep {
    /// Constructs a delay provider using mtimecmp register to sleep
    pub fn new(mtimecmp: MTIMECMP, clocks: Clocks) -> Self {
        Sleep {
            clock_freq: clocks.lfclk().0,
            mtimecmp,
        }
    }
}

impl DelayNs for Sleep {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = (ns as u64) * u64::from(self.clock_freq) / 1_000_000_000;
        let t = MTIME.mtime() + ticks;

        self.mtimecmp.set_mtimecmp(t);

        // Enable timer interrupt
        unsafe { mie::set_mtimer() };

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
        unsafe { mie::clear_mtimer() };
    }
}
