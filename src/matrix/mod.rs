use core::fmt::Write as _;

use embassy_time::Duration;
use embedded_graphics::prelude::*;
use esp_hal::{delay::Delay, rmt::Rmt, time::Rate};
use esp_hal_smartled::SmartLedsAdapter;

pub fn matrix_task(
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

    let handle = esp_rtos::CurrentThreadHandle::get();
    handle.set_priority(31);
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
        embedded_graphics::text::Text::new(buf.as_str(), Point::new(0, 5), style).draw(&mut matrix).ok();
        let now = embassy_time::Instant::now();
        loop {
            matrix.flush_with_gamma().ok();
            Delay::new().delay_millis(50);
            if embassy_time::Instant::now() - now >= Duration::from_millis(100) {
                break;
            }
        }
    }
}
