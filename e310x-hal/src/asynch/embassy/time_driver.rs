//! Embassy time driver implementation.
//!
//! This module provides the driver for embassy-time, keeping track of time and managing the
//! wait queue.
use crate::asynch::delay::{aclint_push_timer, schedule_machine_timer, Timer};
use core::task::Waker;
use e310x::Clint;
use embassy_time_driver::Driver;

struct MyDriver {}

impl Driver for MyDriver {
    fn now(&self) -> u64 {
        // Get the current time from the MTIMER peripheral.
        let mtimer = unsafe { Clint::steal() }.mtimer();
        mtimer.mtime().read()
    }
    fn schedule_wake(&self, at: u64, waker: &Waker) {
        let timer = Timer::new(at, waker.clone());
        let mtimer = unsafe { Clint::steal() }.mtimer();
        let _ = aclint_push_timer(timer);
        // Schedule machine timer interrupt
        schedule_machine_timer(&mtimer);
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: MyDriver = MyDriver{});
