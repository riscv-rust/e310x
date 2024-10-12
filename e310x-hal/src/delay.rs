//! # Delays

use crate::clock::Clocks;
use crate::core::clint::{MTIME, MTIMECMP};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
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

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        let ticks = (us as u64) * TICKS_PER_SECOND / 1_000_000;

        let mtime = MTIME;
        let t = mtime.mtime() + ticks;
        while mtime.mtime() < t {}
    }
}

// This is a workaround to allow `delay_us(42)` construction without specifying a type.
impl DelayUs<i32> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: i32) {
        assert!(us >= 0);
        self.delay_us(us as u32);
    }
}

impl DelayUs<u16> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32::from(us));
    }
}

impl DelayUs<u8> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32::from(us));
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1000);
    }
}

// This is a workaround to allow `delay_ms(42)` construction without specifying a type.
impl DelayMs<i32> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: i32) {
        assert!(ms >= 0);
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u16> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayMs<u8> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms));
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

impl DelayMs<u32> for Sleep {
    fn delay_ms(&mut self, ms: u32) {
        let ticks = (ms as u64) * (self.clock_freq as u64) / 1000;
        let t = MTIME.mtime() + ticks;

        self.mtimecmp.set_mtimecmp(t);

        // Enable timer interrupt
        unsafe {
            mie::set_mtimer();
        }

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

        // Clear timer interrupt
        unsafe {
            mie::clear_mtimer();
        }
    }
}

// This is a workaround to allow `delay_ms(42)` construction without specifying a type.
impl DelayMs<i32> for Sleep {
    #[inline(always)]
    fn delay_ms(&mut self, ms: i32) {
        assert!(ms >= 0);
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u16> for Sleep {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms));
    }
}

impl DelayMs<u8> for Sleep {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms));
    }
}
