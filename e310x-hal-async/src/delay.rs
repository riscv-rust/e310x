//! # Delays
//! # Note
//!
//! Implementation of the asynchronous delay methods for the (A)CLINT peripheral.

use crate::clock::Clocks;
use e310x::CLINT;
use embedded_hal_async::delay::DelayNs;
use riscv::register::mip;
 
mod async_no_mangle {

    static mut TIMER_QUEUE: BinaryHeap<Timer, Min, N_TIMERS> = BinaryHeap::new();

    /// Returns the `MTIMER` register for the current HART ID.
    #[no_mangle]
    fn _riscv_peripheral_aclint_mtimer() -> MTIMER {
        CLINT::mtimer().mtime
    }

    /// Tries to push a new timer to the timer queue assigned to the `MTIMER` register for the current HART ID.
    /// If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
    #[no_mangle]
    fn _riscv_peripheral_aclint_push_timer(t: Timer) -> Result<(), Timer> {
        unsafe { TIMER_QUEUE.push(t) }
    }

    /// Pops all the expired timers from the timer queue assigned to the `MTIMER` register for the
    /// current HART ID and wakes their associated wakers. Once it is done, if the queue is empty,
    /// it returns `None`. Alternatively, if the queue is not empty but the earliest timer has not expired
    /// yet, it returns `Some(next_expires)` where `next_expires` is the tick at which this timer expires.
    #[no_mangle]
    fn _riscv_peripheral_aclint_wake_timers(current_tick: u64) -> Option<u64> {}
}