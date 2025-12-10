//! Async trait definitions replacing the blocking rtc-hal traits.

use rtc_hal::datetime::DateTime;

use crate::error::Error;

/// Async RTC date/time operations.
pub trait AsyncRtc: rtc_hal::error::ErrorType {
    async fn get_datetime(&mut self) -> Result<DateTime, Self::Error>;
    async fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Self::Error>;
}

/// Async power control (start / halt oscillator).
pub trait AsyncRtcPowerControl: rtc_hal::error::ErrorType {
    async fn start_clock(&mut self) -> Result<(), Self::Error>;
    async fn halt_clock(&mut self) -> Result<(), Self::Error>;
}

/// Async NVRAM access.
pub trait AsyncRtcNvram: rtc_hal::error::ErrorType {
    async fn read_nvram(&mut self, offset: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
    async fn write_nvram(&mut self, offset: u8, data: &[u8]) -> Result<(), Self::Error>;
    fn nvram_size(&self) -> u16;
}

/// Supported square wave frequencies (reuse rtc-hal enum).
pub use rtc_hal::square_wave::SquareWaveFreq;

/// Async square wave control.
pub trait AsyncSquareWave: rtc_hal::error::ErrorType {
    async fn start_square_wave(&mut self, freq: SquareWaveFreq) -> Result<(), Self::Error>;
    async fn enable_square_wave(&mut self) -> Result<(), Self::Error>;
    async fn disable_square_wave(&mut self) -> Result<(), Self::Error>;
    async fn set_square_wave_frequency(&mut self, freq: SquareWaveFreq) -> Result<(), Self::Error>;
}
