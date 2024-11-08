//! Basic blinking LEDs example using mtime/mtimecmp registers for "sleep" in a loop.
//! Blinks each led once and goes to the next one.

#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{
        e310x::CLINT,
        i2c::{I2c, Speed},
        prelude::*,
        DeviceResources,
    },
    pin, sprintln,
};
use max3010x::{Led, Max3010x, SampleAveraging};
extern crate panic_halt;

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    let sda = pin!(pins, i2c0_sda).into_iof0();
    let scl = pin!(pins, i2c0_scl).into_iof0();
    let i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    let mut sensor = Max3010x::new_max30102(i2c);
    let part_id = sensor.get_part_id().unwrap();
    sprintln!("Part ID: {:x}", part_id); // This should print "Part ID: 0x15" for a MAX30102.

    let mut sensor = sensor.into_heart_rate().unwrap();
    sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    sensor.set_pulse_amplitude(Led::All, 15).unwrap();
    sensor.enable_fifo_rollover().unwrap();

    let mut data = [0; 3];

    // Get the sleep struct from CLINT
    let mut sleep = CLINT::delay();
    const STEP: u32 = 1000; // 1s
    loop {
        let samples_read = sensor.read_fifo(&mut data).unwrap();
        sprintln!("Samples read: {}", samples_read);
        sleep.delay_ms(STEP);
    }
}
