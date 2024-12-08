//! Stdout
use core::fmt::{Error, Result, Write};

/// Stdout implements the core::fmt::Write trait for embedded_io::Write implementations.
pub struct Stdout<'p, T: 'p>(pub &'p mut T);

impl<T: embedded_io::Write> Write for Stdout<'_, T> {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.as_bytes() {
            if *byte == b'\n' {
                self.0.write(b"\r").map_err(|_| Error)?;
            }
            self.0.write(&[*byte]).map_err(|_| Error)?;
        }
        Ok(())
    }
}
