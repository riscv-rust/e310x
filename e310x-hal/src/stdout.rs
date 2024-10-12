//! Stdout
use core::fmt::Write;
use embedded_hal_nb::serial;
use nb::block;

/// Stdout implements the core::fmt::Write trait for hal::serial::Write
/// implementations.
pub struct Stdout<'p, T: 'p>(pub &'p mut T);

impl<'p, T: serial::Write> Write for Stdout<'p, T> {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.as_bytes() {
            if *byte == b'\n' {
                let res = block!(self.0.write(b'\r'));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }

            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }
        }
        Ok(())
    }
}
