use embassy_futures::select::{select3, Either3};
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Input;

#[embassy_executor::task]
pub async fn adc_task(mut adc: super::Adc, mut battery: super::BatteryPin) {
    loop {
        let battery_adc = adc.read_oneshot(&mut battery);
        match battery_adc {
            Ok(value) => {
                info!("read {} battery ADC value", value);
                let percentage = battery_adc_to_percentage(value);
                info!("Battery: {:.2} %", percentage);
                info!("Battery ADC value: {}", value)
            }
            Err(e) => warn!("Failed to read battery ADC: {:?}", e),
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
