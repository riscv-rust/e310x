//! Basic example of the RC522 RFID reader using the SPI interface.

#![no_std]
#![no_main]

use hifive1::{
    clock,
    hal::{
        e310x::CLINT,
        prelude::*,
        spi::{SpiBus, SpiConfig, MODE_0},
        DeviceResources,
    },
    pin, sprintln,
};
use mfrc522::{comm::blocking::spi::SpiInterface, Mfrc522};
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

    let sck = pin!(pins, spi1_sck).into_iof0();
    let miso = pin!(pins, spi1_miso).into_iof0();
    let mosi = pin!(pins, spi1_mosi).into_iof0();
    let cs = pin!(pins, spi1_ss0).into_iof0();

    let delay = riscv::delay::McycleDelay::new(clocks.coreclk().0);
    let spi_bus = SpiBus::new(p.QSPI1, (mosi, miso, sck, cs));
    let spi_cfg = SpiConfig::new(MODE_0, 1_000_000.hz(), &clocks);
    let spi_device = spi_bus.new_device(&spi_cfg, delay);
    let spi_itf = SpiInterface::new(spi_device);

    let mut mfrc522 = match Mfrc522::new(spi_itf).init() {
        Ok(mfrc522) => mfrc522,
        Err(e) => {
            sprintln!("Error initializing sensor: {:?}", e);
            loop {}
        }
    };
    // The reported version is expected to be 0x91 or 0x92
    let version = mfrc522.version().unwrap();
    sprintln!("Version: {:x}", version);

    // Get the sleep struct from CLINT
    let mut sleep = CLINT::delay();
    const STEP: u32 = 1000; // 1s
    loop {
        sleep.delay_ms(STEP);
    }
}
