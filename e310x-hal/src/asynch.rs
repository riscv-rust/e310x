//! Asynchronous HAL for the E310x family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal-async`] traits for the E310x
//! family of microcontrollers.

#![deny(missing_docs)]

pub mod delay;
pub mod prelude;

// HAL Async Utilities
// async trait implementations for embedded-hal

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
pub use embedded_hal_async::*; // re-export embedded-hal-async to allow macros to use it

/// A future that delegates to a function returning `Poll`.
///
/// This is a wrapper around a function that returns `Poll<T>`.
/// It allows you to create a future that can be polled for a value of type `T`.
pub struct PollFn<F> {
    f: F,
}

/// Creates a future that delegates to a function returning `Poll`.
pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    PollFn { f }
}

impl<T, F> Future for PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        // SAFETY: We're not moving out of the pinned field
        let this = unsafe { self.get_unchecked_mut() };
        (this.f)(cx)
    }
}
