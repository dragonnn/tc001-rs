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
use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::prelude::*;
use embedded_graphics::Drawable;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::rmt::Rmt;
use esp_hal::system::Stack;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal_smartled::SmartLedsAdapterAsync;
use esp_rtos::embassy::Executor;
use log::info;
use smart_leds::SmartLedsWriteAsync;
use static_cell::StaticCell;
extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

pub type Matrix = smart_leds_matrix::SmartLedMatrixAsync<
    SmartLedsAdapterAsync<6400>,
    smart_leds_matrix::layout::Rectangular<smart_leds_matrix::layout::invert_axis::Tc001>,
    { 32 * 8 },
>;

#[embassy_executor::task]
async fn matrix_task(
    rmt: esp_hal::peripherals::RMT<'static>,
    mut led: esp_hal::peripherals::GPIO32<'static>,
) {
    let led = Output::new(led, Level::High, OutputConfig::default());
    embassy_time::Timer::after(Duration::from_millis(100)).await;
    info!("Rmt initializing...");
    let rmt: Rmt<'_, esp_hal::Async> = {
        let frequency: Rate = { Rate::from_mhz(80) };
        Rmt::new(rmt, frequency)
    }
    .expect("Failed to initialize RMT")
    .into_async();
    info!("Rmt initialized.");

    const NUM_LEDS: usize = 32 * 8;
    let rmt_channel = rmt.channel0;
    let rmt_buffer = [0_u32; esp_hal_smartled::buffer_size_async(NUM_LEDS)];

    let mut led = { SmartLedsAdapterAsync::new(rmt_channel, led, rmt_buffer) };

    let mut matrix: smart_leds_matrix::SmartLedMatrixAsync<
        SmartLedsAdapterAsync<_>,
        smart_leds_matrix::layout::Rectangular<smart_leds_matrix::layout::invert_axis::Tc001>,
        _,
    > = smart_leds_matrix::SmartLedMatrixAsync::<_, _, { NUM_LEDS }>::new(
        led,
        smart_leds_matrix::layout::Rectangular::new_tc001(32, 8),
    );
    matrix.set_brightness(32);

    let style = embedded_graphics::mono_font::MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_4X6,
        embedded_graphics::pixelcolor::Rgb888::BLUE,
    );
    matrix.flush_with_gamma().await.ok();
    embassy_time::Timer::after(Duration::from_millis(100)).await;
    info!("Starting matrix loop");
    let mut buf = alloc::string::String::new();
    let mut loops = 0;
    loop {
        info!("alloc: {:?}", esp_alloc::HEAP.stats());
        buf.clear();
        embedded_graphics::primitives::Rectangle::new(Point::new(0, 0), Size::new(32, 8))
            .into_styled(embedded_graphics::primitives::PrimitiveStyle::with_fill(
                embedded_graphics::pixelcolor::Rgb888::BLACK,
            ))
            .draw(&mut matrix)
            .ok();
        write!(&mut buf, "RUST {}", loops).ok();
        embedded_graphics::text::Text::new(buf.as_str(), Point::new(0, 5), style)
            .draw(&mut matrix)
            .ok();
        let now = embassy_time::Instant::now();
        loop {
            matrix.flush_with_gamma().await.ok();
            Timer::after(Duration::from_millis(50)).await;
            if embassy_time::Instant::now() - now > Duration::from_millis(1000) {
                break;
            }
        }
        loops += 1;
        info!("Loop {}", loops);
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();
    info!("Starting up...");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    info!("Peripherals initialized");
    let output_config = esp_hal::gpio::OutputConfig::default();
    let buzzer = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());

    //esp_alloc::heap_allocator!(size: 32 * 1024);

    info!("Heap initialized");
    //COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 96 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    Timer::after(Duration::from_millis(100)).await;

    info!("Embassy initialized!");
    let wifi_init = esp_radio::init().expect("Failed to initialize WIFI/BLE controller");
    /*
               rx_queue_size: 5,
           tx_queue_size: 3,

           static_rx_buf_num: 10,
           dynamic_rx_buf_num: 32,

           static_tx_buf_num: 0,
           dynamic_tx_buf_num: 32,
              rx_ba_win: 6,
    */
    let wifi_config = esp_radio::wifi::WifiConfig::default()
        .with_rx_queue_size(5)
        .with_tx_queue_size(3)
        .with_static_rx_buf_num(10)
        .with_static_tx_buf_num(0)
        .with_rx_ba_win(6);
    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&wifi_init, peripherals.WIFI, wifi_config)
            .expect("Failed to initialize WIFI controller");
    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let ble_config = esp_radio::ble::Config::default();
    let transport =
        esp_radio::ble::controller::BleConnector::new(&wifi_init, peripherals.BT, ble_config);
    let _ble_controller = ExternalController::<_, 5>::new(transport);

    // TODO: Spawn some tasks
    let _ = spawner;

    /*const NUM_LEDS: usize = 32 * 8;
    let rmt_channel = rmt.channel0;
    let rmt_buffer = [0_u32; esp_hal_smartled::buffer_size_async(NUM_LEDS)];

    let mut led = { SmartLedsAdapterAsync::new(rmt_channel, peripherals.GPIO32, rmt_buffer) };

    let mut matrix: smart_leds_matrix::SmartLedMatrixAsync<
        SmartLedsAdapterAsync<_>,
        smart_leds_matrix::layout::Rectangular<smart_leds_matrix::layout::invert_axis::Tc001>,
        _,
    > = smart_leds_matrix::SmartLedMatrixAsync::<_, _, { NUM_LEDS }>::new(
        led,
        smart_leds_matrix::layout::Rectangular::new_tc001(32, 8),
    );
    matrix.set_brightness(32);*/
    let led = peripherals.GPIO32;
    let rmt = peripherals.RMT;
    /*static APP_CORE_STACK: StaticCell<Stack<{ 8 * 1024 }>> = StaticCell::new();
    let app_core_stack = APP_CORE_STACK.init(Stack::new());

    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    esp_rtos::start_second_core(
        peripherals.CPU_CTRL,
        sw_int.software_interrupt0,
        sw_int.software_interrupt1,
        app_core_stack,
        move || {
            static EXECUTOR: StaticCell<Executor> = StaticCell::new();
            let executor = EXECUTOR.init(Executor::new());
            executor.run(|spawner| {
                spawner.spawn(matrix_task(rmt, led)).ok();
            });
        },
    );*/

    spawner.spawn(matrix_task(rmt, led)).ok();

    loop {
        info!("Looping...");
        Timer::after(Duration::from_millis(1000)).await;
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
