#![cfg_attr(not(test), no_std)]

mod async_api;
mod control;
mod datetime;
mod ds1307;
mod error;
mod registers;

pub use async_api::{AsyncRtc, AsyncRtcNvram, AsyncRtcPowerControl, AsyncSquareWave};
pub use ds1307::Ds1307;
pub use rtc_hal::datetime::DateTime;
