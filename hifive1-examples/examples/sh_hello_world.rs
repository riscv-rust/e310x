//! Prints "hello world!" to the host console using semihosting.

#![no_std]
#![no_main]

use semihosting::{println, process::exit};

#[riscv_rt::entry]
fn main() -> ! {
    println!("Hello, world!");
    exit(0);
}
