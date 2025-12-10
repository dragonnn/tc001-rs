use embedded_hal_async::i2c::I2c;
use rtc_hal::{bcd, datetime::DateTimeError};

use crate::{Ds1307, async_api::AsyncRtc, registers::*};

impl<I2C> AsyncRtc for Ds1307<I2C>
where
    I2C: I2c,
{
    /// Async read of the current date and time from the DS1307.
    async fn get_datetime(&mut self) -> Result<rtc_hal::datetime::DateTime, Self::Error> {
        let mut data = [0; 7];
        self.read_register_bytes(Register::Seconds, &mut data).await?;

        // Convert from BCD format and extract fields
        let second = bcd::to_decimal(data[0] & 0b0111_1111); // mask CH (clock halt) bit
        let minute = bcd::to_decimal(data[1]);

        // Handle both 12-hour and 24-hour modes for hours
        let raw_hour = data[2];
        let hour = if (raw_hour & 0b0100_0000) != 0 {
            // 12-hour mode
            // Extract the Hour part (4-0 bits)
            let hr = bcd::to_decimal(raw_hour & 0b0001_1111);
            // Extract the AM/PM (5th bit). if it is set, then it is PM
            let pm = (raw_hour & 0b0010_0000) != 0;

            // Convert it to 24 hour format:
            if pm && hr != 12 {
                hr + 12
            } else if !pm && hr == 12 {
                0
            } else {
                hr
            }
        } else {
            // 24-hour mode
            // Extrac the hour value from 5-0 bits
            bcd::to_decimal(raw_hour & 0b0011_1111)
        };

        let day_of_month = bcd::to_decimal(data[4]);
        let month = bcd::to_decimal(data[5]);
        let year = 2000 + bcd::to_decimal(data[6]) as u16;

        rtc_hal::datetime::DateTime::new(year, month, day_of_month, hour, minute, second)
            .map_err(crate::error::Error::DateTime)
    }

    /// Async set of the current date and time in the DS1307.
    async fn set_datetime(&mut self, datetime: &rtc_hal::datetime::DateTime) -> Result<(), Self::Error> {
        if !(2000..=2099).contains(&datetime.year()) {
            return Err(crate::error::Error::DateTime(DateTimeError::InvalidYear));
        }

        let mut data = [0u8; 8];
        data[0] = Register::Seconds.addr();
        data[1] = bcd::from_decimal(datetime.second()) & 0b0111_1111;
        data[2] = bcd::from_decimal(datetime.minute());
        data[3] = bcd::from_decimal(datetime.hour()) & 0b0011_1111;
        let weekday = datetime.calculate_weekday().map_err(crate::error::Error::DateTime)?;
        data[4] = bcd::from_decimal(weekday.to_number());
        data[5] = bcd::from_decimal(datetime.day_of_month());
        data[6] = bcd::from_decimal(datetime.month());
        let year_2 = (datetime.year() - 2000) as u8;
        data[7] = bcd::from_decimal(year_2);
        self.write_raw_bytes(&data).await?;
        Ok(())
    }
}
