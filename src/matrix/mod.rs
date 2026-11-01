use alloc::vec::Vec;

use embassy_time::Duration;
use embedded_graphics::{prelude::*, primitives::Rectangle};
use esp_hal::{rmt::Rmt, time::Rate, xtensa_lx::interrupt::free};
use esp_hal_smartled::RmtSmartLeds;
use smart_leds::RGB8;

use crate::{adc::get_brightness_percent, state};

mod color;
pub mod event;
mod fonts;
mod pages;
mod status;

#[embassy_executor::task]
pub async fn matrix_task(
    rmt: esp_hal::peripherals::RMT<'static>,
    led: esp_hal::peripherals::GPIO32<'static>,
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
    mut wdt0: crate::Wdt0,
) {
    //let led = Output::new(led, Level::High, OutputConfig::default());
    info!("Rmt initializing...");
    let rmt = {
        let frequency: Rate = { Rate::from_mhz(80) };
        Rmt::new(rmt, frequency)
    }
    .expect("Failed to initialize RMT");
    let rmt_frequency = rmt.frequency();
    info!("Rmt initialized.");

    const NUM_LEDS: usize = 32 * 8;
    const BUFFER_SIZE: usize = esp_hal_smartled::buffer_size::<RGB8>(NUM_LEDS);
    let rmt_channel = rmt.channel0;
    let rmt_buffer = alloc::boxed::Box::<[esp_hal::rmt::PulseCode; BUFFER_SIZE]>::new_zeroed();
    let mut rmt_buffer = unsafe { rmt_buffer.assume_init() };

    type LedColor = RGB8;
    let led = RmtSmartLeds::<
        { esp_hal_smartled::buffer_size::<LedColor>(32 * 8) },
        _,
        LedColor,
        esp_hal_smartled::color_order::Grb,
        esp_hal_smartled::Ws2812bTiming,
    >::new_with_memsize(rmt_channel, led, &mut rmt_buffer, 6, rmt_frequency)
    .expect("Failed to create SmartLeds adapter");
    info!("Led adapter initialized.");

    let mut matrix = smart_leds_matrix::SmartLedMatrix::<_, _, { NUM_LEDS }>::new(
        led,
        smart_leds_matrix::layout::Rectangular::new_tc001(32, 8),
    );

    let handle = esp_rtos::CurrentThreadHandle::get();
    handle.set_priority(31);

    let mut transmission_frames = 0u32;
    let mut transmission_errors = 0u32;
    let mut last_error_report = embassy_time::Instant::now();
    let mut report_transmission = |result: Result<(), esp_hal_smartled::AdapterError>| {
        transmission_frames = transmission_frames.saturating_add(1);
        if let Err(error) = result {
            transmission_errors = transmission_errors.saturating_add(1);
            let now = embassy_time::Instant::now();
            if transmission_errors == 1
                || now
                    .checked_duration_since(last_error_report)
                    .is_some_and(|elapsed| elapsed >= Duration::from_secs(1))
            {
                let rmt = esp_hal::peripherals::RMT::regs();
                let raw = rmt.int_raw().read();
                let status = rmt.chstatus(0).read();
                let config = rmt.chconf0(0).read();
                let limit = rmt.ch_tx_lim(0).read();
                let apb = rmt.apb_conf().read();
                warn!(
                    "Matrix RMT transmission failed: {:?} ({} errors / {} frames)",
                    error, transmission_errors, transmission_frames
                );
                warn!(
                    "Matrix RMT status: raw={:#010x}, status={:#010x}, tx_end={}, hw_error={}, blocks={}, threshold={}, divider={}, wrap={}",
                    raw.bits(), status.bits(), raw.ch_tx_end(0).bit(), raw.ch_err(0).bit(),
                    config.mem_size().bits(), limit.tx_lim().bits(), config.div_cnt().bits(),
                    apb.mem_tx_wrap_en().bit()
                );
                last_error_report = now;
            }
        }
    };

    // Keep this core's interrupts from delaying RMT refills during the blocking flush.
    report_transmission(free(|| matrix.flush_with_gamma()));

    info!("Starting matrix loop");

    //let mut current_page = pages::Time::new(rtc);
    let mut current_page_instant = embassy_time::Instant::now();

    let mut pages = Vec::with_capacity(4);
    pages.push(pages::Time::new(rtc));
    pages.push(pages::Date::new(rtc));
    pages.push(pages::Timer::new(rtc));
    pages.push(pages::Battery::new());

    let mut current_page_index = 0;

    let mut status = status::Status::new();
    let delay_millis = 50;

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
            report_transmission(free(|| matrix.flush_with_gamma()));
            wdt0.feed();
            let now = embassy_time::Instant::now();
            loop {
                embassy_time::Timer::after_millis(delay_millis as u64).await;
                wdt0.feed();
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
            if event.is_single_press() && event.is_long_press() {
                match event.get_main() {
                    event::MatrixEvent::Left => {
                        page_left = true;
                    }
                    event::MatrixEvent::Right => {
                        page_right = true;
                    }
                    event::MatrixEvent::Select => {
                        state::internal_set_transition_state(!state::get_transition_state());
                    }
                }
            } else {
                pages[current_page_index].handle_event(event);
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
                if page_left {
                    for i in 0..size.width {
                        let new_page_offset = Point::new(i as i32 - matrix.size().width as i32, 0);
                        let current_page_offset = Point::new(i as i32, 0);
                        {
                            let mut new_page_target = matrix.translated(new_page_offset);
                            let mut new_page_target = new_page_target.clipped(&Rectangle {
                                top_left: Point::zero(),
                                size: Size { width: size.width - new_page_offset.x as u32, height: size.height },
                            });
                            new_page.render(&mut new_page_target);
                        }
                        {
                            let mut current_page_target = matrix.translated(current_page_offset);
                            let mut current_page_target = current_page_target.clipped(&Rectangle {
                                top_left: Point::zero(),
                                size: Size { width: size.width - current_page_offset.x as u32, height: size.height },
                            });
                            current_page.render(&mut current_page_target);
                        }
                        status.render(&mut matrix);

                        report_transmission(free(|| matrix.flush_with_gamma()));
                        wdt0.feed();
                        embassy_time::Timer::after_millis(25).await;
                    }
                } else {
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

                        report_transmission(free(|| matrix.flush_with_gamma()));
                        wdt0.feed();
                        embassy_time::Timer::after_millis(25).await;
                    }
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
