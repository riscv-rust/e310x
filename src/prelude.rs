//! Prelude

pub use hal::prelude::*;
pub use clint::ClintExt as _e310x_hal_clint_ClintExt;
pub use clock::Clocks;
pub use clock::PrciExt as _e310x_hal_clock_PrciExt;
pub use clock::AonExt as _e310x_hal_clock_AonExt;
pub use gpio::GpioExt as _e310x_hal_gpio_GpioExt;
pub use plic::PlicExt as _e310x_hal_plic_PlicExt;
pub use rtc::RtcExt as _e310x_hal_rtc_RtcExt;
pub use serial::{Serial, Tx, Rx};
pub use time::U32Ext as _e310x_hal_time_U32Ext;
pub use wdog::WdogExt as _e310x_hal_wdog_WdogExt;
