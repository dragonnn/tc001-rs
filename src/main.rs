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

use alloc::boxed::Box;
use alloc::vec;
use bt_hci::controller::ExternalController;
use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_time::{Duration, Timer};
use embedded_graphics::prelude::*;
use embedded_graphics::Drawable;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::rmt::Rmt;
use esp_hal::rng::Rng;
use esp_hal::system::Stack;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal_smartled::SmartLedsAdapter;
use esp_hal_smartled::SmartLedsAdapterAsync;
use esp_radio::wifi::ModeConfig;
use esp_radio::{
    wifi::{ClientConfig, Config, ScanConfig, WifiController, WifiDevice, WifiEvent, WifiStaState},
    Controller,
};
use esp_rtos::embassy::Executor;
use esp_rtos::embassy::InterruptExecutor;
use log::error;
use log::info;
use smart_leds::SmartLedsWriteAsync;
use static_cell::StaticCell;
extern crate alloc;

mod sntpc;

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

#[embassy_executor::task]
async fn matrix_task(
    rmt: esp_hal::peripherals::RMT<'static>,
    mut led: esp_hal::peripherals::GPIO32<'static>,
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
) {
    let led = Output::new(led, Level::High, OutputConfig::default());
    embassy_time::Timer::after(Duration::from_millis(100)).await;
    info!("async Rmt initializing...");
    let rmt: Rmt<'_, esp_hal::Async> = {
        let frequency: Rate = { Rate::from_mhz(80) };
        Rmt::new(rmt, frequency)
    }
    .expect("Failed to initialize RMT")
    .into_async();
    info!("async Rmt initialized.");

    const NUM_LEDS: usize = 32 * 8;
    const BUFFER_SIZE: usize = esp_hal_smartled::buffer_size_async(NUM_LEDS);
    let rmt_channel = rmt.channel0;
    //let mut rmt_buffer = [0_u32; esp_hal_smartled::buffer_size_async(NUM_LEDS)];
    let rmt_buffer = alloc::boxed::Box::<[u32; BUFFER_SIZE]>::new_zeroed();
    let mut rmt_buffer = unsafe { rmt_buffer.assume_init() };

    let mut led: SmartLedsAdapterAsync<'_, BUFFER_SIZE> =
        { SmartLedsAdapterAsync::new(rmt_channel, led, rmt_buffer.as_mut_array().unwrap()) };

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

    let handle = esp_rtos::CurrentThreadHandle::get();
    handle.set_priority(30);
    info!("Matrix task started: {:?}", handle);

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
        let now = rtc.current_time_us();
        let now = chrono::NaiveDateTime::from_timestamp_micros(now as i64).unwrap();
        write!(&mut buf, "{}", now.time()).ok();
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

fn matrix_blocking(
    rmt: esp_hal::peripherals::RMT<'static>,
    mut led: esp_hal::peripherals::GPIO32<'static>,
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
) {
    //let led = Output::new(led, Level::High, OutputConfig::default());
    info!("Rmt initializing...");
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        let frequency: Rate = { Rate::from_mhz(80) };
        Rmt::new(rmt, frequency)
    }
    .expect("Failed to initialize RMT");
    info!("Rmt initialized.");

    const NUM_LEDS: usize = 32 * 8;
    const BUFFER_SIZE: usize = esp_hal_smartled::buffer_size_async(NUM_LEDS);
    let rmt_channel = rmt.channel0;
    //let mut rmt_buffer = [0_u32; esp_hal_smartled::buffer_size_async(NUM_LEDS)];
    let rmt_buffer = alloc::boxed::Box::<[u32; BUFFER_SIZE]>::new_zeroed();
    let mut rmt_buffer = unsafe { rmt_buffer.assume_init() };

    info!("Rmt buffer initialized: {:?}", rmt_buffer.len());

    let mut led: SmartLedsAdapter<'_, BUFFER_SIZE> =
        { SmartLedsAdapter::new(rmt_channel, led, rmt_buffer.as_mut_array().unwrap()) };
    info!("Led adapter initialized.");

    let mut matrix = smart_leds_matrix::SmartLedMatrix::<_, _, { NUM_LEDS }>::new(
        led,
        smart_leds_matrix::layout::Rectangular::new_tc001(32, 8),
    );
    info!("Matrix initialized.");
    matrix.set_brightness(32);
    info!("Matrix brightness set.");

    let style = embedded_graphics::mono_font::MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_4X6,
        embedded_graphics::pixelcolor::Rgb888::BLUE,
    );

    //let handle = esp_rtos::CurrentThreadHandle::get();
    //handle.set_priority(31);
    //info!("Matrix task started: {:?}", handle);

    matrix.flush_with_gamma().ok();

    info!("Starting matrix loop");
    let mut buf = alloc::string::String::new();
    loop {
        buf.clear();
        embedded_graphics::primitives::Rectangle::new(Point::new(0, 0), Size::new(32, 8))
            .into_styled(embedded_graphics::primitives::PrimitiveStyle::with_fill(
                embedded_graphics::pixelcolor::Rgb888::BLACK,
            ))
            .draw(&mut matrix)
            .ok();
        let now = rtc.current_time_us();
        let now = chrono::NaiveDateTime::from_timestamp_micros(now as i64).unwrap();
        write!(&mut buf, "{}", now.time()).ok();
        info!("Time: {}", buf);
        embedded_graphics::text::Text::new(buf.as_str(), Point::new(0, 5), style)
            .draw(&mut matrix)
            .ok();
        let now = embassy_time::Instant::now();
        loop {
            matrix.flush_with_gamma().ok();
            Delay::new().delay_millis(50);
            if embassy_time::Instant::now() - now >= Duration::from_millis(250) {
                break;
            }
        }
    }
}

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

    //esp_alloc::heap_allocator!(size: 32 * 1024);

    info!("Heap initialized");
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 96 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    Timer::after(Duration::from_millis(1000)).await;

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
            //let spawner = executor_core1.start(esp_hal::interrupt::Priority::Priority3);

            //spawner.spawn(matrix_task(rmt, led, rtc2)).ok();
            //loop {}
            matrix_blocking(rmt, led, rtc2);
        },
    );

    //spawner.spawn(matrix_task(rmt, led)).ok();

    info!("Waiting 10s before initializing wifi...");
    embassy_time::Timer::after(Duration::from_secs(20)).await;
    /*
               rx_queue_size: 5,
           tx_queue_size: 3,

           static_rx_buf_num: 10,
           dynamic_rx_buf_num: 32,

           static_tx_buf_num: 0,
           dynamic_tx_buf_num: 32,
              rx_ba_win: 6,
    */
    let wifi_config = esp_radio::wifi::Config::default()
        .with_rx_queue_size(2)
        .with_tx_queue_size(2)
        .with_static_rx_buf_num(2)
        .with_static_tx_buf_num(0)
        .with_rx_ba_win(3);

    let esp_radio_ctrl = &*mk_static!(Controller<'static>, esp_radio::init().unwrap());

    let (mut wifi_controller, interfaces) =
        esp_radio::wifi::new(&esp_radio_ctrl, peripherals.WIFI, wifi_config)
            .expect("Failed to initialize WIFI controller");
    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    /*let ble_config = esp_radio::ble::Config::default();
    let transport =
        esp_radio::ble::controller::BleConnector::new(&esp_radio_ctrl, peripherals.BT, ble_config);
    let _ble_controller = ExternalController::<_, 5>::new(transport);*/

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

    let wifi_interface = interfaces.sta;

    let config = embassy_net::Config::dhcpv4(Default::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    //spawner.spawn(matrix_task(rmt, led)).ok();

    spawner.spawn(wifi_connection(wifi_controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    let date = ntp_request(&stack).await.ok();
    info!("NTP date: {:?}", date);
    if let Some(date) = date {
        rtc2.set_current_time_us(date.and_utc().timestamp_micros() as u64);
    }

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

const SSID: &str = dotenvy_macro::dotenv!("WIFI_SSID");
const PASSWORD: &str = dotenvy_macro::dotenv!("WIFI_PASSWORD");

#[embassy_executor::task]
async fn wifi_connection(mut controller: WifiController<'static>) {
    info!("start connection task");
    info!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_radio::wifi::sta_state() {
            WifiStaState::Connected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = ModeConfig::Client(
                ClientConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into()),
            );
            controller.set_config(&client_config).unwrap();
            info!("Starting wifi");
            controller.start_async().await.unwrap();
            info!("Wifi started!");

            info!("Scan");
            let scan_config = ScanConfig::default().with_max(10);
            let result = controller
                .scan_with_config_async(scan_config)
                .await
                .unwrap();
            for ap in result {
                info!("{:?}", ap);
            }
        }
        info!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => {
                info!("Failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

use embassy_net::{
    dns::DnsQueryType,
    udp::{PacketMetadata, UdpSocket},
    IpEndpoint,
};

struct UdpBuffers {
    rx_meta: Box<[PacketMetadata]>,
    rx_buffer: Box<[u8]>,
    tx_meta: Box<[PacketMetadata]>,
    tx_buffer: Box<[u8]>,
}

impl UdpBuffers {
    pub fn new() -> Self {
        Self {
            rx_meta: Box::new([PacketMetadata::EMPTY; 16]),
            rx_buffer: Box::new([0; 2048]),
            tx_meta: Box::new([PacketMetadata::EMPTY; 16]),
            tx_buffer: Box::new([0; 2048]),
        }
    }

    pub fn as_static_mut(
        &mut self,
    ) -> (
        &'static mut [PacketMetadata],
        &'static mut [u8],
        &'static mut [PacketMetadata],
        &'static mut [u8],
    ) {
        unsafe {
            core::mem::transmute((
                self.rx_meta.as_mut(),
                self.rx_buffer.as_mut(),
                self.tx_meta.as_mut(),
                self.tx_buffer.as_mut(),
            ))
        }
    }
}

async fn ntp_request<'d>(stack: &'d embassy_net::Stack<'d>) -> Result<chrono::NaiveDateTime, ()> {
    info!("Prepare NTP request");
    let mut addrs = stack
        .dns_query("pl.pool.ntp.org", smoltcp::wire::DnsQueryType::A)
        .await
        .unwrap_or_default();
    let addr = addrs.pop().ok_or(())?;
    info!("NTP DNS: {:?}", addr);

    let ntp_packet = sntpc::NtpPacket::new();
    let raw_ntp = sntpc::RawNtpPacket::from(&ntp_packet);

    let mut buffers = UdpBuffers::new();

    let (rx_meta, rx_buffer, tx_meta, tx_buffer) = buffers.as_static_mut();

    let mut socket =
        embassy_net::udp::UdpSocket::new(*stack, rx_meta, rx_buffer, tx_meta, tx_buffer);

    socket.bind(11770).map_err(|e| ())?;
    info!("UDP socket bound");

    socket
        .send_to(&raw_ntp.0, IpEndpoint::new(addr, 123))
        .await
        .map_err(|e| ())?;
    info!("NTP request sent");

    let mut buffer = [0u8; 48];

    socket.recv_from(&mut buffer).await.map_err(|e| ())?;

    let mut raw_ntp = sntpc::RawNtpPacket::default();
    raw_ntp.0 = buffer;

    let recv_timestamp = embassy_time::Instant::now();
    let recv_timestamp = sntpc::get_ntp_timestamp(&recv_timestamp);

    let result = sntpc::process_response(ntp_packet.into(), raw_ntp, recv_timestamp);

    match result {
        Ok(packet) => match packet.to_datetime() {
            Some(datetime) => {
                let datetime = datetime.with_timezone(&chrono_tz::Europe::Warsaw);
                // info!("NTP time received: {:?}", defmt::Debug2Format(&datetime));

                return Ok(datetime.naive_local());
            }
            None => {
                error!("Failed to convert NTP packet to NaiveDateTime");
            }
        },
        Err(e) => {
            error!("Failed to process NTP response: {:?}", e);
        }
    }

    Err(())
}
