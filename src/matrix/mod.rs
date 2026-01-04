use alloc::boxed::Box;
use core::fmt::Write as _;

use embassy_time::Duration;
use embedded_graphics::{prelude::*, primitives::Rectangle};
use esp_hal::{
    delay::Delay,
    rmt::{PulseCode, Rmt},
    time::Rate,
};
use esp_hal_smartled::SmartLedsAdapter;

mod event;
mod fonts;
mod pages;

pub fn matrix_task(
    rmt: esp_hal::peripherals::RMT<'static>,
    mut led: esp_hal::peripherals::GPIO32<'static>,
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
    mut wdt0: crate::Wdt0,
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
    let rmt_buffer = alloc::boxed::Box::<[PulseCode; BUFFER_SIZE]>::new_zeroed();
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
    matrix.set_brightness(16);
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

    let mut current_page = pages::Time::new(rtc);
    let mut current_page_instant = embassy_time::Instant::now();

    loop {
        wdt0.feed();
        current_page.update();
        current_page.render(&mut matrix);
        let now = embassy_time::Instant::now();
        loop {
            matrix.flush_with_gamma().ok();
            wdt0.feed();
            Delay::new().delay_millis(50);
            let now2 = embassy_time::Instant::now();
            if let Some(elapsed) = now2.checked_duration_since(now) {
                if elapsed >= Duration::from_millis(100) {
                    break;
                }
            } else {
                warn!("Time went backwards!");
                break;
            }
        }

        let now = embassy_time::Instant::now();
        if let Some(elapsed) = now.checked_duration_since(current_page_instant) {
            if elapsed >= Duration::from_secs(10) {
                let mut new_page = match current_page {
                    pages::Pages::Time(_) => pages::Battery::new(),
                    pages::Pages::Battery(_) => pages::Date::new(rtc),
                    pages::Pages::Date(_) => pages::Time::new(rtc),
                };

                new_page.update();

                let size = matrix.size();
                for i in (0..size.width).rev() {
                    let current_page_offset = Point::new(i as i32 - matrix.size().width as i32, 0);
                    let new_page_offset = Point::new(i as i32, 0);

                    {
                        let mut current_page_target = matrix.translated(current_page_offset);
                        let mut current_page_target = current_page_target.clipped(&Rectangle {
                            top_left: Point::zero(),
                            size: Size { width: size.width - current_page_offset.x as u32, height: size.height },
                        });
                        current_page.render(&mut current_page_target);
                    }
                    {
                        let mut new_page_target = matrix.translated(new_page_offset);
                        let mut new_page_target = new_page_target.clipped(&Rectangle {
                            top_left: Point::zero(),
                            size: Size { width: size.width - new_page_offset.x as u32, height: size.height },
                        });
                        new_page.render(&mut new_page_target);
                    }

                    matrix.flush_with_gamma().ok();
                    wdt0.feed();
                    Delay::new().delay_millis(25);
                }

                current_page = new_page;
                current_page_instant = embassy_time::Instant::now();
            }
        } else {
            warn!("Time went backwards!");
            current_page_instant = embassy_time::Instant::now();
        }
    }
}
