use rtc_hal::datetime::DateTimeError;

/// DS1307 driver errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<I2cError>
where
    I2cError: core::fmt::Debug,
{
    /// I2C communication error
    I2c(I2cError),
    /// Invalid register address
    InvalidAddress,
    /// The specified square wave frequency is not supported by the RTC
    UnsupportedSqwFrequency,
    /// Invalid date/time parameters provided by user
    DateTime(DateTimeError),
    /// NVRAM write would exceed available space
    NvramOutOfBounds,
}

impl<I2cError> From<I2cError> for Error<I2cError>
where
    I2cError: core::fmt::Debug,
{
    fn from(value: I2cError) -> Self {
        Error::I2c(value)
    }
}

impl<I2cError> rtc_hal::error::Error for Error<I2cError>
where
    I2cError: core::fmt::Debug,
{
    fn kind(&self) -> rtc_hal::error::ErrorKind {
        match self {
            Error::I2c(_) => rtc_hal::error::ErrorKind::Bus,
            Error::InvalidAddress => rtc_hal::error::ErrorKind::InvalidAddress,
            Error::DateTime(_) => rtc_hal::error::ErrorKind::InvalidDateTime,
            Error::NvramOutOfBounds => rtc_hal::error::ErrorKind::NvramOutOfBounds,
            Error::UnsupportedSqwFrequency => rtc_hal::error::ErrorKind::UnsupportedSqwFrequency,
        }
    }
}
