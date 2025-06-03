//! # Delays
//! # Note
//!
//! Implementation of the asynchronous delay methods for the (A)CLINT peripheral.

use core::cell::RefCell;
use critical_section::Mutex;
use e310x::CLINT;
use heapless::binary_heap::{BinaryHeap, Min};
use riscv_peripheral::{aclint::mtimer::MTIMER, hal_async::aclint::Timer};

pub use riscv_peripheral::hal_async::aclint::DelayNs;

const N_TIMERS: usize = 16;
static TIMER_QUEUE: Mutex<RefCell<BinaryHeap<Timer, Min, N_TIMERS>>> =
    Mutex::new(RefCell::new(BinaryHeap::new()));

/// Returns the `MTIMER` register for the current HART ID.
#[unsafe(no_mangle)]
fn _riscv_peripheral_aclint_mtimer() -> MTIMER {
    CLINT::mtimer()
}

/// Tries to push a new timer to the timer queue assigned to the `MTIMER` register for the current HART ID.
/// If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
#[unsafe(no_mangle)]
fn _riscv_peripheral_aclint_push_timer(t: Timer) -> Result<(), Timer> {
    critical_section::with(|cs| {
        let timer_queue = &mut *TIMER_QUEUE.borrow_ref_mut(cs);
        timer_queue.push(t)
    })
}

/// Pops all the expired timers from the timer queue assigned to the `MTIMER` register for the
/// current HART ID and wakes their associated wakers. Once it is done, if the queue is empty,
/// it returns `None`. Alternatively, if the queue is not empty but the earliest timer has not expired
/// yet, it returns `Some(next_expires)` where `next_expires` is the tick at which this timer expires.
#[unsafe(no_mangle)]
fn _riscv_peripheral_aclint_wake_timers(current_tick: u64) -> Option<u64> {
    critical_section::with(|cs| {
        let timer_queue = &mut *TIMER_QUEUE.borrow_ref_mut(cs);
        let mut next_expires = None;
        while let Some(t) = timer_queue.peek() {
            if t.expires() > current_tick {
                next_expires = Some(t.expires());
                break;
            }
            let t = timer_queue.pop().unwrap();
            t.waker().wake_by_ref();
        }
        next_expires
    })
}
