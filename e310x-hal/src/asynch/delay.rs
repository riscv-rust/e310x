//! Asynchronous delay implementation for the (A)CLINT peripheral.
//!
//! # Note
//!
//! The asynchronous delay implementation for the (A)CLINT peripheral relies on the machine-level timer interrupts.
//! Therefore, it needs to schedule the machine-level timer interrupts via the [`MTIMECMP`] register assigned to the current HART.
//! Thus, the [`Delay`] instance must be created on the same HART that is used to call the asynchronous delay methods.
//!
//! # Requirements
//!
//! The following `extern "Rust"` functions must be implemented:
//!
//! - `fn _riscv_peripheral_aclint_mtimer(hart_id: usize) -> MTIMER`: This function returns the `MTIMER` register for the given HART ID.
//! - `fn _riscv_peripheral_aclint_push_timer(t: Timer) -> Result<(), Timer>`: This function pushes a new timer to a timer queue assigned to the given HART ID.
//!   If it fails (e.g., the timer queue is full), it returns back the timer that failed to be pushed.
//!   The logic of timer queues are application-specific and are not provided by this crate.
//! - `fn _riscv_peripheral_aclint_wake_timers(current_tick: u64) -> Option<u64>`:
//!   This function pops all the expired timers from a timer queue assigned to the current HART ID and wakes their associated wakers.
//!   The function returns the next [`MTIME`] tick at which the next timer expires. If the queue is empty, it returns `None`.

pub use crate::hal_async::delay::DelayNs;
use crate::{
    aclint::mtimer::{Mtimer, MTIMER},
    hal_async::poll_fn,
};
use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    task::{Poll, Waker},
};

/// Machine-level timer interrupt handler. This handler is triggered whenever the `MTIME`
/// register reaches the value of the `MTIMECMP` register of the HART in charge of waking the timers.
#[export_name = "MachineTimer"]
fn machine_timer() {
    schedule_machine_timer();
}

/// Schedules the next machine timer interrupt for the given HART ID according to the timer queue.
#[inline]
fn schedule_machine_timer() {
    let current_tick = unsafe { _riscv_peripheral_aclint_mtime_read() };
    if let Some(next_expires) = unsafe { _riscv_peripheral_aclint_wake_timers(current_tick) } {
        debug_assert!(next_expires > current_tick);
        unsafe {
            _riscv_peripheral_aclint_mtimecmp_write(next_expires);
        }
    }
}

impl<M: Mtimer> MTIMER<M> {
    #[inline]
    async fn delay_ticks(&mut self, n_ticks: u64) {
        let expires = self.mtime().read() + n_ticks;
        let mut pushed = false;
        poll_fn(move |cx| {
            if self.mtime().read() < expires {
                if !pushed {
                    // Push timer to queue only on first pending poll
                    pushed = true;
                    let timer = Timer::new(expires, cx.waker().clone());
                    unsafe { _riscv_peripheral_aclint_push_timer(timer) };
                    // Schedule machine timer interrupt
                    schedule_machine_timer();
                }
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }
}

impl<M: Mtimer> DelayNs for MTIMER<M> {
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        let n_ticks = ns as u64 * M::MTIME_FREQ as u64 / 1_000_000_000;
        self.delay_ticks(n_ticks).await;
    }

    #[inline]
    async fn delay_us(&mut self, us: u32) {
        let n_ticks = us as u64 * M::MTIME_FREQ as u64 / 1_000_000;
        self.delay_ticks(n_ticks).await;
    }

    #[inline]
    async fn delay_ms(&mut self, ms: u32) {
        let n_ticks = ms as u64 * M::MTIME_FREQ as u64 / 1_000;
        self.delay_ticks(n_ticks).await;
    }
}

/// Timer queue entry.
///
/// When pushed to the timer queue via the `_riscv_peripheral_aclint_push_timer` function,
/// this entry provides the necessary information to adapt it to the timer queue implementation.
#[derive(Debug)]
pub struct Timer {
    expires: u64,
    waker: Waker,
}

impl Timer {
    /// Creates a new timer queue entry.
    #[inline]
    const fn new(expires: u64, waker: Waker) -> Self {
        Self { expires, waker }
    }

    /// Returns the expiration tick of the timer.
    #[inline]
    pub const fn expires(&self) -> u64 {
        self.expires
    }

    /// Consumes the timer and wakes its associated waker.
    #[inline]
    pub fn wake(self) {
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
        Some(self.expires.cmp(&other.expires))
    }
}
