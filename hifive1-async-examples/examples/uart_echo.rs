//! Simple echo example for the asynchronous UART.
//! This example uses asynchronous UART.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use hifive1::{
    clock,
    hal::{
        asynch::prelude::*,
        e310x::Uart0,
        gpio::{
            gpio0::{Pin16, Pin17},
            NoInvert, IOF0,
        },
        prelude::*,
        serial::{Rx, Serial, Tx},
        DeviceResources,
    },
};
extern crate panic_halt;

static SHARED_CHANNEL: Channel<CriticalSectionRawMutex, ([u8; 20], usize), 1> = Channel::new();

/// Continuously read the received bytes and send them over the channel.
#[embassy_executor::task]
async fn read_task(mut rx: Rx<Uart0, Pin16<IOF0<NoInvert>>>) {
    loop {
        // Temporary buffer
        let mut buf = [0u8; 20];

        // Read
        let count = _eioa_Read::read(&mut rx, &mut buf).await.unwrap();

        // Send the filled buffer to the echo task
        SHARED_CHANNEL.send((buf, count)).await;
    }
}

/// Wait for a message on the channel and echo it back over UART Tx.
#[embassy_executor::task]
async fn echo_task(mut tx: Tx<Uart0, Pin17<IOF0<NoInvert>>>) {
    loop {
        // Receive a buffer from the reader
        let (buf, count) = SHARED_CHANNEL.receive().await;

        // Echo back
        _eioa_Write::write_all(&mut tx, &buf[..count])
            .await
            .unwrap();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let dr = DeviceResources::take().unwrap();
    let cp = dr.core_peripherals;
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART
    let tx = pins.pin17.into_iof0();
    let rx = pins.pin16.into_iof0();
    let mut serial = Serial::new(p.UART0, (tx, rx), 115_200.bps(), clocks);

    // Make sure interrupts are disabled
    riscv::interrupt::disable();

    // Reset PLIC interrupts and set priority threshold
    let plic = cp.plic;
    let priorities = plic.priorities();
    let ctx = plic.ctx0();
    priorities.reset::<ExternalInterrupt>();
    unsafe {
        ctx.enables().disable_all::<ExternalInterrupt>();
        ctx.threshold().set_threshold(Priority::P0);
    }

    // Enable the UART0 interrupt source
    unsafe {
        serial.set_exti_priority(&plic, Priority::P1);
        serial.enable_exti(&plic);
    }

    // Enable global interrupts
    unsafe {
        riscv::interrupt::enable();
        plic.enable();
    }

    // Split the serial into Tx and Rx parts
    let (tx, rx) = serial.split();

    // Spawn the read and write tasks
    spawner.spawn(read_task(rx)).unwrap();
    spawner.spawn(echo_task(tx)).unwrap();
}
