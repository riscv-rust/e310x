//! Prelude

pub use embedded_hal::prelude::*;
pub use crate::clint::ClintExt as _e310x_hal_clint_ClintExt;
pub use crate::clock::Clocks;
pub use crate::clock::PrciExt as _e310x_hal_clock_PrciExt;
pub use crate::clock::AonExt as _e310x_hal_clock_AonExt;
pub use crate::gpio::GpioExt as _e310x_hal_gpio_GpioExt;
pub use crate::plic::PlicExt as _e310x_hal_plic_PlicExt;
pub use crate::rtc::RtcExt as _e310x_hal_rtc_RtcExt;
pub use crate::serial::{Serial, Tx, Rx};
pub use crate::time::U32Ext as _e310x_hal_time_U32Ext;
pub use crate::wdog::WdogExt as _e310x_hal_wdog_WdogExt;
