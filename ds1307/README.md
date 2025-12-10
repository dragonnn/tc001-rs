# DS1307 RTC Driver (Async)

Async Rust driver for the DS1307 using embedded-hal-async.

## Basic usage

```rust
use ds1307::{Ds1307, DateTime};
use ds1307::async_api::{AsyncRtc, AsyncRtcPowerControl};

async fn example<I2C>(i2c: I2C) -> Result<(), ds1307::error::Error<I2C::Error>>
where
    I2C: embedded_hal_async::i2c::I2c,
    I2C::Error: core::fmt::Debug,
{
    let mut rtc = Ds1307::new(i2c);
    rtc.start_clock().await?;
    let set = DateTime::new(2025, 8, 21, 14, 30, 0).unwrap();
    rtc.set_datetime(&set).await?;
    let now = rtc.get_datetime().await?;
    let _ = now;
    Ok(())
}
```

## License

This project is licensed under the MIT License.
