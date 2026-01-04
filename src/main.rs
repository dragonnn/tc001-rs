#![feature(impl_trait_in_assoc_type)]
#![feature(slice_as_array)]
#![feature(new_zeroed_alloc)]
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#[macro_use]
extern crate log;

use alloc::{boxed::Box, vec};
use core::fmt::Write;

use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_time::{Duration, Timer};
use embedded_graphics::{prelude::*, Drawable};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    i2c::master::I2c,
    interrupt::software::SoftwareInterruptControl,
    rmt::Rmt,
    rng::Rng,
    system::Stack,
    time::Rate,
    timer::timg::TimerGroup,
};
use esp_hal_smartled::{SmartLedsAdapter, SmartLedsAdapterAsync};
use esp_rtos::embassy::{Executor, InterruptExecutor};
use log::{error, info};
use smart_leds::SmartLedsWriteAsync;
use static_cell::StaticCell;
extern crate alloc;

mod adc;
mod buttons;
mod ds1307;
mod ha;
mod heap;
mod matrix;
mod mk_static;
mod mqtt;
mod ntp;
mod storage;
mod udp;
mod wifi;

pub type BatteryPin =
    esp_hal::analog::adc::AdcPin<esp_hal::peripherals::GPIO34<'static>, esp_hal::peripherals::ADC1<'static>>;
pub type LightSensorPin =
    esp_hal::analog::adc::AdcPin<esp_hal::peripherals::GPIO35<'static>, esp_hal::peripherals::ADC1<'static>>;
pub type Adc = esp_hal::analog::adc::Adc<'static, esp_hal::peripherals::ADC1<'static>, esp_hal::Blocking>;
pub type Wdt0 = esp_hal::timer::timg::Wdt<esp_hal::peripherals::TIMG0<'static>>;
pub type I2c0 = embassy_sync::mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    I2c<'static, esp_hal::Async>,
>;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();
    info!("Starting up tc001 v3");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    info!("Peripherals initialized");
    let output_config = esp_hal::gpio::OutputConfig::default();
    let buzzer = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut wdt0 = timg0.wdt;
    wdt0.set_timeout(esp_hal::timer::timg::MwdtStage::Stage0, esp_hal::time::Duration::from_millis(500));
    wdt0.enable();
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    info!("Heap initialized");
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 96 * 1024);

    //6 - SDA GPIO21
    //7 - SCL GPIO22
    let sda = peripherals.GPIO21;
    let scl = peripherals.GPIO22;

    let i2c_config = esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100));

    let i2c0 = &*mk_static::mk_static!(
        I2c0,
        embassy_sync::mutex::Mutex::new(
            esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config)
                .unwrap()
                .with_scl(scl)
                .with_sda(sda)
                .into_async()
        )
    );

    let mut rtc = esp_hal::rtc_cntl::Rtc::new(peripherals.LPWR);
    info!("estimate_xtal_frequency: {}", rtc.estimate_xtal_frequency());
    static RTC: StaticCell<esp_hal::rtc_cntl::Rtc> = StaticCell::new();
    let rtc = RTC.init(rtc);

    spawner.must_spawn(ds1307::ds1307_task(i2c0, rtc));

    Timer::after(Duration::from_millis(10)).await;

    spawner.must_spawn(heap::heap_task());

    info!("Embassy initialized!");
    let led = peripherals.GPIO32;
    let rmt = peripherals.RMT;
    static APP_CORE_STACK: StaticCell<Stack<{ 8 * 1024 }>> = StaticCell::new();
    let app_core_stack = APP_CORE_STACK.init(Stack::new());

    let rtc2 = &*rtc;
    info!("Starting second core...");
    esp_rtos::start_second_core(peripherals.CPU_CTRL, sw_int.software_interrupt1, app_core_stack, move || {
        matrix::matrix_task(rmt, led, rtc2, wdt0);
    });

    let wifi_config = esp_radio::wifi::Config::default()
        .with_rx_queue_size(2)
        .with_tx_queue_size(2)
        .with_static_rx_buf_num(2)
        .with_static_tx_buf_num(1)
        .with_rx_ba_win(3);

    //let esp_radio_ctrl = &*mk_static::mk_static!(Controller<'static>, esp_radio::init().unwrap());

    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, wifi_config).expect("Failed to initialize WIFI controller");

    let wifi_interface = interfaces.station;

    let config = embassy_net::Config::dhcpv4(Default::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static::mk_static!(StackResources<4>, StackResources::<4>::new()),
        seed,
    );

    let storage = storage::init(peripherals.FLASH).await;

    spawner.must_spawn(wifi::wifi_task(wifi_controller, *&storage));
    spawner.must_spawn(wifi::net_task(runner));

    let left = Input::new(peripherals.GPIO26, InputConfig::default().with_pull(Pull::Up));
    let right = Input::new(peripherals.GPIO27, InputConfig::default().with_pull(Pull::Up));
    let middle = Input::new(peripherals.GPIO14, InputConfig::default().with_pull(Pull::Up));

    spawner.must_spawn(ntp::ntp_task(stack));
    spawner.must_spawn(mqtt::mqtt_task(stack));
    spawner.must_spawn(ha::ha_task(spawner, stack));
    spawner.must_spawn(buttons::button_task(left, right, middle));

    let mut adc_config = esp_hal::analog::adc::AdcConfig::default();

    let battery_pin = adc_config.enable_pin(peripherals.GPIO34, esp_hal::analog::adc::Attenuation::_11dB);
    let light_sensor_pin = adc_config.enable_pin(peripherals.GPIO35, esp_hal::analog::adc::Attenuation::_11dB);

    let adc = esp_hal::analog::adc::Adc::new(peripherals.ADC1, adc_config);

    spawner.must_spawn(adc::adc_task(adc, battery_pin, light_sensor_pin));

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
