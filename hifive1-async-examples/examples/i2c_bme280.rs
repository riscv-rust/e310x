//! Sample example for the BME280 temperature, pressure and humidity sensor.
//! using the I2C interface.
//! This example uses synchronous UART and only tests asynchronous I2C.

#![no_std]
#![no_main]

use bme280_rs::{AsyncBme280, Configuration, Filter, Oversampling, SensorMode};
use embassy_executor::Spawner;
use hifive1::{
    clock,
    hal::{
        asynch::delay::Delay,
        asynch::prelude::*,
        e310x::interrupt::Hart,
        i2c::{I2c, Speed},
        prelude::*,
        DeviceResources,
    },
    sprintln,
};
extern crate panic_halt;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let cp = dr.core_peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pins.pin17, pins.pin16, 115_200.bps(), clocks);

    sprintln!("Configuring I2C...");

    // I2C configuration
    let sda = pins.pin12.into_iof0();
    let scl = pins.pin13.into_iof0();
    let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    // Disable and clear pending I2C interrupts from previous states
    i2c.disable_interrupt();
    i2c.clear_interrupt();

    // Get the MTIMER peripheral from CLINT
    let mtimer = cp.clint.mtimer();
    mtimer.disable();
    let (mtimecmp, mtime) = (mtimer.mtimecmp(Hart::H0), mtimer.mtime());
    mtime.write(0);
    mtimecmp.write(u64::MAX);
    let mut delay = Delay::new(mtimer);
    const STEP: u32 = 1000; // 1s

    sprintln!("Configuring external interrupts...");

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

    // Enable I2C0 interrupt source
    unsafe {
        i2c.set_exti_priority(&plic, Priority::P1);
        i2c.enable_exti(&plic);
    }

    sprintln!("Enabling external interrupts...");

    // Enable global interrupts
    unsafe {
        riscv::interrupt::enable();
        plic.enable();
    }

    sprintln!("Configuring BME280 sensor...");

    // BME280 sensor configuration
    let mut bme280 = AsyncBme280::new(i2c, delay.clone());
    bme280.init().await.unwrap();
    bme280
        .set_sampling_configuration(
            Configuration::default()
                .with_temperature_oversampling(Oversampling::Oversample16)
                .with_pressure_oversampling(Oversampling::Oversample16)
                .with_humidity_oversampling(Oversampling::Oversample16)
                .with_filter(Filter::Off)
                .with_sensor_mode(SensorMode::Forced),
        )
        .await
        .unwrap();

    sprintln!("Measuring...");

    // Execute loop
    loop {
        // Measure
        bme280.take_forced_measurement().await.unwrap();
        let sample = bme280.read_sample().await.unwrap();

        // Retrieve the returned temperature as °C, pressure in Pa and humidity in %RH
        sprintln!(
            "Current measurement: {:.2} Celsius, {:.2} Pa, {:.2}%RH",
            sample.temperature.unwrap(),
            sample.pressure.unwrap(),
            sample.humidity.unwrap()
        );

        delay.delay_ms(STEP).await;
    }
}
