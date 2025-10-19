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
    gpio::{Level, Output, OutputConfig},
    interrupt::software::SoftwareInterruptControl,
    rmt::Rmt,
    rng::Rng,
    system::Stack,
    time::Rate,
    timer::timg::TimerGroup,
};
use esp_hal_smartled::{SmartLedsAdapter, SmartLedsAdapterAsync};
use esp_radio::{
    wifi::{ClientConfig, Config, ModeConfig, ScanConfig, WifiController, WifiDevice, WifiEvent, WifiStaState},
    Controller,
};
use esp_rtos::embassy::{Executor, InterruptExecutor};
use log::{error, info};
use smart_leds::SmartLedsWriteAsync;
use static_cell::StaticCell;
extern crate alloc;

mod heap;
mod matrix;
mod ntp;
mod udp;
mod wifi;

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

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

    info!("Heap initialized");
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 96 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    spawner.must_spawn(heap::heap_task());

    info!("Embassy initialized!");
    let led = peripherals.GPIO32;
    let rmt = peripherals.RMT;
    static APP_CORE_STACK: StaticCell<Stack<{ 8 * 1024 }>> = StaticCell::new();
    let app_core_stack = APP_CORE_STACK.init(Stack::new());

    //let handle = esp_rtos::CurrentThreadHandle::get();
    //handle.set_priority(30);

    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    static EXECUTOR_CORE_1: StaticCell<InterruptExecutor<2>> = StaticCell::new();
    let executor_core1 = InterruptExecutor::new(sw_int.software_interrupt2);
    let executor_core1 = EXECUTOR_CORE_1.init(executor_core1);

    let rtc = esp_hal::rtc_cntl::Rtc::new(peripherals.LPWR);
    static RTC: StaticCell<esp_hal::rtc_cntl::Rtc> = StaticCell::new();
    let rtc = RTC.init(rtc);

    let rtc2 = &*rtc;
    info!("Starting second core...");
    esp_rtos::start_second_core(
        peripherals.CPU_CTRL,
        sw_int.software_interrupt0,
        sw_int.software_interrupt1,
        app_core_stack,
        move || {
            matrix::matrix_task(rmt, led, rtc2);
        },
    );

    let wifi_config = esp_radio::wifi::Config::default()
        .with_rx_queue_size(2)
        .with_tx_queue_size(2)
        .with_static_rx_buf_num(2)
        .with_static_tx_buf_num(1)
        .with_rx_ba_win(3);

    let esp_radio_ctrl = &*mk_static!(Controller<'static>, esp_radio::init().unwrap());

    let (wifi_controller, interfaces) = esp_radio::wifi::new(&esp_radio_ctrl, peripherals.WIFI, wifi_config)
        .expect("Failed to initialize WIFI controller");

    let wifi_interface = interfaces.sta;

    let config = embassy_net::Config::dhcpv4(Default::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    let (stack, runner) =
        embassy_net::new(wifi_interface, config, mk_static!(StackResources<3>, StackResources::<3>::new()), seed);

    spawner.must_spawn(wifi::wifi_task(wifi_controller));
    spawner.must_spawn(wifi::net_task(runner));

    spawner.must_spawn(ntp::ntp_task(stack, rtc2));

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
