//! # Delays

use embedded_hal::blocking::delay::DelayMs;
use crate::clint::{MTIME, MTIMECMP};
use riscv::register::{mie, mip};

/// Machine timer (mtime) as a busyloop delay provider
pub struct Delay;

impl Delay {
    /// Constructs a delay provider based on the machine timer (mtime)
    pub fn new() -> Self {
        Delay
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        let ticks = (ms as u64) * 65536 / 1000;

        let mtime = MTIME;
        let t = mtime.mtime() + ticks;
        while mtime.mtime() < t { }
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms));
    }
}

/// Machine timer (mtime) as a sleep delay provider using mtimecmp
pub struct Sleep {
    mtimecmp: MTIMECMP
}

impl Sleep {
    /// Constructs a delay provider using mtimecmp register to sleep
    pub fn new(mtimecmp: MTIMECMP) -> Self {
        unsafe {
            mie::set_mtimer();
        }

        Sleep {
            mtimecmp
        }
    }
}

impl DelayMs<u32> for Sleep {
    fn delay_ms(&mut self, ms: u32) {
        let ticks = (ms as u64) * 65536 / 1000;
        let t = MTIME.mtime() + ticks;

        self.mtimecmp.set_mtimecmp(t);
        
        // Wait For Interrupt will put CPU to sleep until an interrupt hits
        // in our case when internal timer mtime value >= mtimecmp value
        // after which empty handler gets called and we go into the
        // next iteration of this loop
        loop {
            unsafe {
                riscv::asm::wfi();
            }

            // check if we got the right interrupt cause, otherwise just loop back to wfi
            if mip::read().mtimer() {
                break;
            }
        }
    }
}

impl DelayMs<u16> for Sleep {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayMs<u8> for Sleep {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms));
    }
}
