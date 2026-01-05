use core::sync::atomic::{AtomicU16, Ordering::Relaxed};

use embassy_time::{Duration, Timer};
use num_traits::Pow as _;

static BATTERY_LEVEL_PERCENTAGE: AtomicU16 = AtomicU16::new(0);
static BRIGHTNESS_PERCENT: AtomicU16 = AtomicU16::new(0);

const MIN_BATTERY_RAW: u16 = 575;
const MAX_BATTERY_RAW: u16 = 656;
const MIN_BRIGHTNESS: u16 = 2;
const MAX_BRIGHTNESS: u16 = 100;
const LDR_GAMMA: f32 = 3.0;
const LDR_FACTOR: f32 = 1.0;

fn map_range<T>(value: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Copy
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Div<Output = T>
        + core::cmp::PartialOrd,
{
    let out = (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    if out < out_min {
        out_min
    } else if out > out_max {
        out_max
    } else {
        out
    }
}

#[embassy_executor::task]
pub async fn adc_task(mut adc: super::Adc, mut battery: super::BatteryPin, mut light_sensor: super::LightSensorPin) {
    loop {
        let mut battery_total: u32 = 0;
        let mut light_sensor_total: u32 = 0;
        for _i in 0..10 {
            let battery_adc = read_oneshot(&mut adc, &mut battery).await;
            battery_total += battery_adc as u32;
            let light_adc = read_oneshot(&mut adc, &mut light_sensor).await;
            light_sensor_total += light_adc as u32;
            Timer::after(Duration::from_millis(10)).await;
        }
        let battery_avg = (battery_total / 10) as u16;
        let light_sensor_avg = (light_sensor_total / 10) as u16;

        let battery_percentage = battery_adc_to_percentage(battery_avg);
        BATTERY_LEVEL_PERCENTAGE.store((battery_percentage * 10.0) as u16, Relaxed);

        let brightness_percent = map_range(
            ((light_sensor_avg as f32 * LDR_FACTOR) / 1023.0 * 100.0).pow(LDR_GAMMA) / 100.0f32.pow(LDR_GAMMA - 1.0),
            MIN_BRIGHTNESS as f32,
            MAX_BRIGHTNESS as f32,
            0.0,
            100.0,
        );

        BRIGHTNESS_PERCENT.store((brightness_percent * 10.0) as u16, Relaxed);

        Timer::after(Duration::from_secs(1)).await;
    }
}

fn battery_adc_to_percentage(adc_value: u16) -> f32 {
    let clamped_value = if adc_value < MIN_BATTERY_RAW {
        MIN_BATTERY_RAW
    } else if adc_value > MAX_BATTERY_RAW {
        MAX_BATTERY_RAW
    } else {
        adc_value
    };
    map_range(clamped_value as f32, MIN_BATTERY_RAW as f32, MAX_BATTERY_RAW as f32, 0.0, 100.0)
}

fn scale_12_to_10_exact(x: u16) -> u16 {
    let x = (x & 0x0FFF) as u32;
    ((x * 1023 + 2047) / 4095) as u16
}

async fn read_oneshot<'d, PIN>(
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
                Timer::after(Duration::from_millis(5)).await;
            }
        }
    }
}

pub fn get_battery_level_percentage() -> f32 {
    BATTERY_LEVEL_PERCENTAGE.load(Relaxed) as f32 / 10.0
}

pub fn get_brightness_percent() -> f32 {
    BRIGHTNESS_PERCENT.load(Relaxed) as f32 / 10.0
}
