//! Prelude

pub use crate::clock::AonExt as _e310x_hal_clock_AonExt;
pub use crate::clock::PrciExt as _e310x_hal_clock_PrciExt;
pub use crate::gpio::GpioExt as _e310x_hal_gpio_GpioExt;
pub use crate::rtc::RtcExt as _e310x_hal_rtc_RtcExt;
pub use crate::time::U32Ext as _e310x_hal_time_U32Ext;
pub use crate::wdog::WdogExt as _e310x_hal_wdog_WdogExt;
pub use e310x::interrupt::{
    CoreInterrupt, Exception, ExceptionNumber, ExternalInterrupt, InterruptNumber, Priority,
    PriorityNumber,
};
pub use embedded_hal::{
    self,
    delay::DelayNs as _eh_DelayNs,
    digital::{InputPin, OutputPin, StatefulOutputPin},
    i2c::I2c as _embedded_hal_i2c_I2c,
    pwm::SetDutyCycle,
    spi::{SpiBus, SpiDevice},
};

pub use embedded_hal_nb::{
    self,
    serial::{Read as _embedded_hal_nb_Read, Write as _embedded_hal_nb_Write},
    spi::FullDuplex,
};

pub use embedded_io;
