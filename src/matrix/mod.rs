use alloc::{boxed::Box, vec::Vec};
use core::fmt::Write as _;

use embassy_time::Duration;
use embedded_graphics::{prelude::*, primitives::Rectangle};
use esp_hal::{
    delay::Delay,
    rmt::{PulseCode, Rmt},
    time::Rate,
};
use esp_hal_smartled::SmartLedsAdapter;

use crate::{adc::get_brightness_percent, state};

mod color;
pub mod event;
mod fonts;
mod pages;
mod status;

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

    let led: SmartLedsAdapter<'_, BUFFER_SIZE> =
        { SmartLedsAdapter::new(rmt_channel, led, rmt_buffer.as_mut_array().unwrap()) };
    info!("Led adapter initialized.");

    let mut matrix = smart_leds_matrix::SmartLedMatrix::<_, _, { NUM_LEDS }>::new(
        led,
        smart_leds_matrix::layout::Rectangular::new_tc001(32, 8),
    );

    let handle = esp_rtos::CurrentThreadHandle::get();
    handle.set_priority(31);

    matrix.flush_with_gamma().ok();

    info!("Starting matrix loop");

    //let mut current_page = pages::Time::new(rtc);
    let mut current_page_instant = embassy_time::Instant::now();

    let mut pages = Vec::with_capacity(4);
    pages.push(pages::Time::new(rtc));
    pages.push(pages::Date::new(rtc));
    pages.push(pages::Battery::new());

    let mut current_page_index = 0;

    let mut status = status::Status::new();
    let mut delay_millis = 50;

    let event_receiver = event::get_event_channel_receiver();

    loop {
        let event = event_receiver.try_receive();
        if event.is_err() {
            let current_page = &mut pages[current_page_index];
            let mut brightness = ((get_brightness_percent() / 100.0) * 255.0) as u8;
            if brightness < 5 {
                brightness = 5;
            }
            matrix.set_brightness(brightness);
            wdt0.feed();
            current_page.update();
            current_page.render(&mut matrix);
            status.update();
            status.render(&mut matrix);
            let now = embassy_time::Instant::now();
            loop {
                matrix.flush_with_gamma().ok();
                wdt0.feed();
                Delay::new().delay_millis(delay_millis);
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
        }
        let mut page_left = false;
        let mut page_right = false;
        if let Ok(event) = event {
            match event {
                event::MatrixEvent::PageLeft => {
                    page_left = true;
                }
                event::MatrixEvent::PageRight => {
                    page_right = true;
                }
                _ => {
                    warn!("Unexpected event received in matrix task: {:?}", event);
                }
            }
        }

        let transition_state = state::get_transition_state();
        let now = embassy_time::Instant::now();
        if let Some(elapsed) = now.checked_duration_since(current_page_instant) {
            if (elapsed >= Duration::from_secs(10) && transition_state) || (page_left || page_right) {
                let mut new_page_index = (current_page_index + 1) % pages.len();
                if page_left {
                    if current_page_index == 0 {
                        new_page_index = pages.len() - 1;
                    } else {
                        new_page_index = current_page_index - 1;
                    }
                }

                let [new_page, current_page] = pages.get_disjoint_mut([new_page_index, current_page_index]).unwrap();

                //let new_page = &mut pages[new_page_index];
                //let current_page = &mut pages[current_page_index];

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

                    status.render(&mut matrix);

                    matrix.flush_with_gamma().ok();
                    wdt0.feed();
                    Delay::new().delay_millis(25);
                }

                current_page_index = new_page_index;
                current_page_instant = embassy_time::Instant::now();
            } else if !transition_state {
                current_page_instant = embassy_time::Instant::now();
            }
        } else {
            warn!("Time went backwards!");
            current_page_instant = embassy_time::Instant::now();
        }

        for (page_index, page) in &mut pages.iter_mut().enumerate() {
            if page_index != current_page_index {
                page.idle_update();
            }
        }
    }
}
