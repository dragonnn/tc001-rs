use embassy_futures::select::{select3, Either3};
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Input;

#[embassy_executor::task]
pub async fn adc_task(mut adc: super::Adc, mut battery: super::BatteryPin, mut light_sensor: super::LightSensorPin) {
    loop {
        for i in 0..10 {
            let battery_adc = read_oneshot(&mut adc, &mut battery).await;

            info!("read {} battery ADC value", battery_adc);
            let percentage = battery_adc_to_percentage(battery_adc);
            info!("bat: {:.2} %", percentage);

            let light_adc = read_oneshot(&mut adc, &mut light_sensor).await;

            info!("read {} light sensor ADC value", light_adc);

            Timer::after(Duration::from_millis(100)).await;
        }
        Timer::after(Duration::from_secs(10)).await;
    }
}

fn battery_adc_to_percentage(adc_value: u16) -> f32 {
    let adc_value = adc_value as f32 * 1.6;
    info!("adc_value: {}", adc_value);
    let percentage = (adc_value - 3.0) / 0.69 * 100.0;
    info!("percentage: {}", percentage);
    percentage.max(100.0).min(0.0)
}

fn scale_12_to_10_exact(x: u16) -> u16 {
    let x = (x & 0x0FFF) as u32;
    ((x * 1023 + 2047) / 4095) as u16
}

pub async fn read_oneshot<'d, PIN>(
    adc: &mut super::Adc,
    pin: &mut esp_hal::analog::adc::AdcPin<PIN, esp_hal::peripherals::ADC1<'static>>,
) -> u16
where
    PIN: esp_hal::analog::adc::AdcChannel,
{
    loop {
        match adc.read_oneshot(pin) {
            Ok(value) => return scale_12_to_10_exact(value),
            Err(_) => {
                Timer::after(Duration::from_millis(10)).await;
            }
        }
    }
}
