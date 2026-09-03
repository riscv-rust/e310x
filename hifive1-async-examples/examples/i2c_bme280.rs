//! Sample example for the BME280 temperature, pressure and humidity sensor.
//! using the I2C interface.
//! This example uses synchronous UART and only tests asynchronous I2C.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embedded_devices::devices::bosch::bme280::registers::{IIRFilter, Oversampling};
use embedded_devices::devices::bosch::bme280::{address::Address, BME280Async, Configuration};
use embedded_devices::sensor::OneshotSensorAsync;
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
use uom::si::{pressure::pascal, ratio::percent, thermodynamic_temperature::degree_celsius};
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

    // I2C configuration
    let sda = pins.pin12.into_iof0();
    let scl = pins.pin13.into_iof0();
    let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    // Get the MTIMER peripheral from CLINT
    let mtimer = cp.clint.mtimer();
    mtimer.disable();
    let (mtimecmp, mtime) = (mtimer.mtimecmp(Hart::H0), mtimer.mtime());
    mtime.write(0);
    mtimecmp.write(u64::MAX);
    let mut delay = Delay::new(mtimer);
    const STEP: u32 = 1000; // 1s

    // Set button interrupt source priority
    let plic = cp.plic;
    let priorities = plic.priorities();
    priorities.reset::<ExternalInterrupt>();
    unsafe { i2c.set_exti_priority(&plic, Priority::P1) };

    // Enable interrupts
    let ctx = plic.ctx0();
    unsafe {
        ctx.enables().disable_all::<ExternalInterrupt>();
        i2c.enable_exti(&plic);
        ctx.threshold().set_threshold(Priority::P0);
        riscv::interrupt::enable();
        plic.enable();
    };

    //BME280 sensor configuration
    let bme280_delay = Delay::new(mtimer);
    let mut bme280 = BME280Async::new_i2c(bme280_delay, i2c, Address::Primary);
    bme280.init().await.unwrap();
    bme280
        .configure(Configuration {
            temperature_oversampling: Oversampling::X_16,
            pressure_oversampling: Oversampling::X_16,
            humidity_oversampling: Oversampling::X_16,
            iir_filter: IIRFilter::Disabled,
        })
        .await
        .unwrap();
    loop {
        // Measure
        let measurement = bme280.measure().await.unwrap();

        // Retrieve the returned temperature as °C, pressure in Pa and humidity in %RH
        let temp = measurement.temperature.get::<degree_celsius>();
        let pressure = measurement.pressure.unwrap().get::<pascal>();
        let humidity = measurement.humidity.unwrap().get::<percent>();
        sprintln!(
            "Current measurement: {:.2} Celsius, {:.2} Pa, {:.2}%RH",
            temp,
            pressure,
            humidity
        );

        delay.delay_ms(STEP).await;
    }
}
