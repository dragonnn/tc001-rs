#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use alloc::vec;
use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::prelude::GrayColor as _;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::RgbColor as _;
use embedded_graphics::Drawable;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal_smartled::SmartLedsAdapterAsync;
use esp_wifi::ble::controller::BleConnector;
use log::info;
use smart_leds::SmartLedsWriteAsync;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init =
        esp_wifi::init(timer1.timer0, rng).expect("Failed to initialize WIFI/BLE controller");
    let (mut _wifi_controller, _interfaces) = esp_wifi::wifi::new(&wifi_init, peripherals.WIFI)
        .expect("Failed to initialize WIFI controller");
    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let transport = BleConnector::new(&wifi_init, peripherals.BT);
    let _ble_controller = ExternalController::<_, 5>::new(transport);

    // TODO: Spawn some tasks
    let _ = spawner;

    let output_config = esp_hal::gpio::OutputConfig::default();
    let buzzer = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    info!("Rmt initializing...");
    let rmt: Rmt<'_, esp_hal::Async> = {
        let frequency: Rate = { Rate::from_mhz(80) };
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT")
    .into_async();
    info!("Rmt initialized.");

    const NUM_LEDS: usize = 32 * 8;
    let rmt_channel = rmt.channel0;
    let rmt_buffer = [0_u32; esp_hal_smartled::buffer_size_async(NUM_LEDS)];

    let mut led = { SmartLedsAdapterAsync::new(rmt_channel, peripherals.GPIO32, rmt_buffer) };

    let mut matrix = smart_leds_matrix::SmartLedMatrixAsync::<_, _, { NUM_LEDS }>::new(
        led,
        smart_leds_matrix::layout::Rectangular::new_tc001(32, 8),
    );
    let style = embedded_graphics::mono_font::MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_5X7,
        embedded_graphics::pixelcolor::Rgb888::CYAN,
    );

    loop {
        embedded_graphics::text::Text::new("RUST!", Point::new(3, 6), style)
            .draw(&mut matrix)
            .ok();
        matrix.flush_with_gamma().await.ok();
        Timer::after(Duration::from_millis(10)).await;
        /*
        for mut data in data {
            for hue in 0..=255 {
                color.hue = hue;
                // Convert from the HSV color space (where we can easily transition from one
                // color to the other) to the RGB color space that we can then send to the LED
                data = smart_leds::hsv::hsv2rgb(color);
                // When sending to the LED, we do a gamma correction first (see smart_leds
                // documentation for details) and then limit the brightness to 10 out of 255 so
                // that the output is not too bright.
                led.write(smart_leds::brightness(
                    smart_leds::gamma([data].into_iter()),
                    level,
                ))
                .await
                .unwrap();
                Timer::after(Duration::from_millis(10)).await;
            }
        }*/
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
