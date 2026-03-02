//! Asynchronous delay implementation for the (A)CLINT peripheral.
//!
//! # Note
//!
//! The asynchronous delay implementation for the (A)CLINT peripheral relies on the machine-level timer interrupts.
//! Therefore, it needs to schedule the machine-level timer interrupts via the [`MTIMECMP`] register assigned to the current HART.
//! Thus, the [`Delay`] instance must be created on the same HART that is used to call the asynchronous delay methods.

use core::{
    cell::RefCell,
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    future::poll_fn,
    task::{Poll, Waker},
};
use critical_section::Mutex;
use e310x::Clint;
use embedded_hal_async::delay::DelayNs;
use heapless::binary_heap::{BinaryHeap, Min};
use riscv_peripheral::aclint::mtimer::MTIMER;

const N_TIMERS: usize = 16;
static TIMER_QUEUE: Mutex<RefCell<BinaryHeap<Timer, Min, N_TIMERS>>> =
    Mutex::new(RefCell::new(BinaryHeap::new()));

/// Tries to push a new timer to the timer queue assigned to the `MTIMER` register for the current HART ID.
/// If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
#[inline]
pub(crate) fn aclint_push_timer(t: Timer) -> Result<(), Timer> {
    critical_section::with(|cs| {
        let mut timer_queue = TIMER_QUEUE.borrow_ref_mut(cs);
        timer_queue.push(t)
    })
}

/// Pops all the expired timers from the timer queue assigned to the `MTIMER` register for the
/// current HART ID and wakes their associated wakers. Once it is done, if the queue is empty,
/// it returns `None`. Alternatively, if the queue is not empty but the earliest timer has not expired
/// yet, it returns `Some(next_expires)` where `next_expires` is the tick at which this timer expires.
#[inline]
fn aclint_wake_timers(current_tick: u64) -> Option<u64> {
    critical_section::with(|cs| {
        let mut timer_queue = TIMER_QUEUE.borrow_ref_mut(cs);
        let mut next_expires = None;
        while let Some(t) = timer_queue.peek() {
            if t.expires() > current_tick {
                next_expires = Some(t.expires());
                break;
            }
            let t = unsafe { timer_queue.pop_unchecked() };
            t.wake();
        }
        next_expires
    })
}

/// Machine-level timer interrupt handler. This handler is triggered whenever the `MTIME`
/// register reaches the value of the `MTIMECMP` register of the HART in charge of waking the timers.
#[riscv_rt::core_interrupt(e310x::interrupt::CoreInterrupt::MachineTimer)]
fn machine_timer() {
    let clint = unsafe { Clint::steal() };
    let mtimer = clint.mtimer();
    schedule_machine_timer(&mtimer);
}

/// Schedules the next machine timer interrupt for the given HART ID according to the timer queue.
#[inline]
pub(crate) fn schedule_machine_timer(mtimer: &MTIMER<Clint>) {
    let current_tick = mtimer.mtime().read();
    mtimer.disable();
    if let Some(next_expires) = aclint_wake_timers(current_tick) {
        debug_assert!(next_expires > current_tick);
        mtimer.mtimecmp0().write(next_expires);
        unsafe { mtimer.enable() };
    }
}

/// Timer queue entry.
///
/// When pushed to the timer queue via the `aclint_push_timer` function,
/// this entry provides the necessary information to adapt it to the timer queue implementation.
#[derive(Debug)]
pub(crate) struct Timer {
    expires: u64,
    waker: Waker,
}

impl Timer {
    /// Creates a new timer queue entry.
    #[inline]
    pub(crate) const fn new(expires: u64, waker: Waker) -> Self {
        Self { expires, waker }
    }

    /// Returns the expiration tick of the timer.
    #[inline]
    const fn expires(&self) -> u64 {
        self.expires
    }

    /// Consumes the timer and wakes its associated waker.
    #[inline]
    fn wake(self) {
        self.waker.wake();
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires
    }
}

impl Eq for Timer {}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.expires.cmp(&other.expires)
    }
}

impl PartialOrd for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// async delay trait implementation from `embedded-hal-async`
#[derive(Clone)]
pub struct Delay {
    mtimer: MTIMER<Clint>,
}

impl Delay {
    /// Creates a new delay instance.
    #[inline]
    pub fn new(mtimer: MTIMER<Clint>) -> Self {
        Self { mtimer }
    }

    /// Delays for the given number of ticks.
    #[inline]
    async fn delay_ticks(&mut self, n_ticks: u64) {
        let mtime = self.mtimer.mtime();
        let expires = mtime.read() + n_ticks;
        let mut pushed = false;
        poll_fn(move |cx| {
            if mtime.read() < expires {
                if !pushed {
                    // Push timer to queue only on first pending poll
                    pushed = true;
                    let timer = Timer::new(expires, cx.waker().clone());
                    let _ = aclint_push_timer(timer);
                    // Schedule machine timer interrupt
                    schedule_machine_timer(&self.mtimer);
                }
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

impl DelayNs for Delay {
    /// Delays for the given number of nanoseconds.
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        let n_ticks = ns as u64 * self.mtimer.mtime_freq() as u64 / 1_000_000_000;
        self.delay_ticks(n_ticks).await;
    }

    /// Delays for the given number of microseconds.
    #[inline]
    async fn delay_us(&mut self, us: u32) {
        let n_ticks = us as u64 * self.mtimer.mtime_freq() as u64 / 1_000_000;
        self.delay_ticks(n_ticks).await;
    }

    /// Delays for the given number of milliseconds.
    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        let n_ticks = ms as u64 * self.mtimer.mtime_freq() as u64 / 1_000;
        self.delay_ticks(n_ticks).await;
    }
}
