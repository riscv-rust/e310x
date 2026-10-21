//! Sample example for the BME280 temperature, pressure and humidity sensor.
//! using the SPI interface.
//! This example uses synchronous UART and only tests asynchronous SPI.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embedded_devices::{
    devices::bosch::bme280::{
        registers::{IIRFilter, Oversampling},
        BME280Async, Configuration,
    },
    sensor::OneshotSensorAsync,
    uom::si::{pressure::pascal, ratio::percent, thermodynamic_temperature::degree_celsius},
};
use hifive1::{
    clock,
    hal::{
        asynch::delay::Delay,
        asynch::prelude::*,
        e310x::interrupt::Hart,
        prelude::*,
        spi::{SpiBus, SpiConfig, MODE_0},
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

    sprintln!("Configuring SPI...");

    // SPI configuration
    let sck = pins.pin5.into_iof0();
    let miso = pins.pin4.into_iof0();
    let mosi = pins.pin3.into_iof0();
    let cs = pins.pin2.into_iof0();
    let mut spi_bus = SpiBus::new(p.QSPI1, (mosi, miso, sck, cs));
    let spi_cfg = SpiConfig::new(MODE_0, 1_000_000.hz(), &clocks);

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

    // Enable QSPI1 interrupt source
    unsafe {
        spi_bus.set_exti_priority(&plic, Priority::P1);
        spi_bus.enable_exti(&plic);
    }

    sprintln!("Enabling external interrupts...");

    // Enable global interrupts
    unsafe {
        riscv::interrupt::enable();
        plic.enable();
    }

    sprintln!("Configuring BME280 sensor...");

    // SPI device configuration
    let spi_device = spi_bus.new_device_async(&spi_cfg, delay.clone());

    // BME280 sensor configuration
    let mut bme280 = BME280Async::new_spi(delay.clone(), spi_device);
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

    sprintln!("Measuring...");

    // Execute loop
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
