//! Stdout based on the UART hooked up to FTDI or J-Link

use core::{
    fmt::{self, Write as CoreWrite},
    ptr,
};
use e310x_hal::{
    clock::Clocks,
    e310x::Uart0,
    gpio::{
        gpio0::{Pin16, Pin17},
        NoInvert, IOF0,
    },
    prelude::*,
    serial::{Rx, Serial, Tx},
    time::Bps,
};
use nb::block;

struct SerialWrapper(Tx<Uart0, Pin17<IOF0<NoInvert>>>);

static mut STDOUT: Option<SerialWrapper> = None;

impl CoreWrite for SerialWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if *byte == b'\n' {
                let res = block!(self.0.write(b'\r'));

                if res.is_err() {
                    return Err(fmt::Error);
                }
            }

            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(fmt::Error);
            }
        }
        Ok(())
    }
}

/// Configures stdout
pub fn configure<X, Y>(
    uart: Uart0,
    tx: Pin17<X>,
    rx: Pin16<Y>,
    baud_rate: Bps,
    clocks: Clocks,
) -> Rx<Uart0, Pin16<IOF0<NoInvert>>> {
    let tx = tx.into_iof0();
    let rx = rx.into_iof0();
    let serial = Serial::new(uart, (tx, rx), baud_rate, clocks);
    let (tx, rx) = serial.split();

    critical_section::with(|_| {
        unsafe { &mut *ptr::addr_of_mut!(STDOUT) }.replace(SerialWrapper(tx));
    });
    rx
}

/// Writes string to stdout
pub fn write_str(s: &str) {
    critical_section::with(|_| {
        if let Some(stdout) = unsafe { &mut *ptr::addr_of_mut!(STDOUT) } {
            let _ = stdout.write_str(s);
        }
    });
}

/// Writes formatted string to stdout
pub fn write_fmt(args: fmt::Arguments) {
    critical_section::with(|_| {
        if let Some(stdout) = unsafe { &mut *ptr::addr_of_mut!(STDOUT) } {
            let _ = stdout.write_fmt(args);
        }
    });
}

/// Macro for printing to the serial standard output
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        $crate::stdout::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the serial standard output, with a newline.
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::stdout::write_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
