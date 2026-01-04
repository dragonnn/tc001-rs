use chrono::{Datelike as _, Timelike as _};
use ds1307::{AsyncRtc as _, AsyncRtcPowerControl as _};
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c;
use esp_hal::gpio::Input;

use crate::ntp::wait_for_ntp_sync;

#[embassy_executor::task]
pub async fn ds1307_task(mut i2c0: &'static crate::I2c0, rtc: &'static esp_hal::rtc_cntl::Rtc<'static>) {
    let i2c_device = embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice::new(&mut i2c0);
    let mut ds1307 = ds1307::Ds1307::new(i2c_device);
    ds1307.start_clock().await.unwrap();
    sync_ds1307_to_rtc(&mut ds1307, rtc).await;
    loop {
        match select(Timer::after_secs(1), wait_for_ntp_sync()).await {
            Either::First(_) => {
                sync_ds1307_to_rtc(&mut ds1307, rtc).await;
            }
            Either::Second(ntp_datetime) => {
                info!(
                    "Setting DS1307 DateTime from NTP: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    ntp_datetime.year(),
                    ntp_datetime.month(),
                    ntp_datetime.day(),
                    ntp_datetime.hour(),
                    ntp_datetime.minute(),
                    ntp_datetime.second()
                );
                let ds1307_datetime = ds1307::DateTime::new(
                    ntp_datetime.year() as u16,
                    ntp_datetime.month() as u8,
                    ntp_datetime.day() as u8,
                    ntp_datetime.hour() as u8,
                    ntp_datetime.minute() as u8,
                    ntp_datetime.second() as u8,
                )
                .unwrap();
                ds1307.set_datetime(&ds1307_datetime).await.unwrap();
            }
        }
    }
}

pub async fn sync_ds1307_to_rtc<I2C: I2c>(
    ds1307: &mut ds1307::Ds1307<
        embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice<'static, CriticalSectionRawMutex, I2C>,
    >,
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
) {
    let datetime = ds1307.get_datetime().await;
    if let Ok(datetime) = datetime {
        info!(
            "DS1307 DateTime: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            datetime.year(),
            datetime.month(),
            datetime.day_of_month(),
            datetime.hour(),
            datetime.minute(),
            datetime.second()
        );
        let datetime = chrono::NaiveDate::from_ymd_opt(
            datetime.year() as i32,
            datetime.month() as u32,
            datetime.day_of_month() as u32,
        )
        .unwrap()
        .and_hms_opt(datetime.hour() as u32, datetime.minute() as u32, datetime.second() as u32)
        .unwrap();

        rtc.set_current_time_us(datetime.and_utc().timestamp_micros() as u64);
    } else {
        error!("Failed to read DS1307 DateTime: {:?}", datetime.err());
    }
}
