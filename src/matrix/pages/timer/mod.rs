use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use embassy_time::Instant;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use crate::matrix::{event::MatrixEventDetails, fonts::AwtrixFont, pages::Pages};

static POMODOR_ICON: &[u8] = include_bytes!("./pomodoro.gif");

pub struct Timer {
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
    icon: tinygif::Gif<'static>,
    last_frame: usize,
    current_frame: usize,
    current_frame_draw: Instant,
    remaining_time_ms: u32,

    timer_started_at: Option<chrono::NaiveDateTime>,
    timer_duration: chrono::Duration,

    buf: String,
}

impl Timer {
    pub fn new(rtc: &'static esp_hal::rtc_cntl::Rtc<'static>) -> Pages {
        Pages::Timer(Box::new(Timer {
            rtc,
            icon: tinygif::Gif::from_slice(POMODOR_ICON).unwrap(),
            current_frame: 0,
            last_frame: usize::MAX,
            current_frame_draw: Instant::now(),
            remaining_time_ms: 0,
            timer_started_at: None,
            timer_duration: chrono::Duration::minutes(15),

            buf: String::with_capacity(5),
        }))
    }

    pub fn update(&mut self) {}

    pub fn render<T: super::PageTarget>(&mut self, target: &mut T) {
        target.clear(Rgb888::BLACK).ok();
        self.buf.clear();
        //
        //let font = embedded_graphics::mono_font::ascii::FONT_4X6;
        //let style = embedded_graphics::mono_font::MonoTextStyle::new(&font, embedded_graphics::pixelcolor::Rgb888::RED);
        //let style = super::font::mateine(embedded_graphics::pixelcolor::Rgb888::RED);
        //let style = AwtrixFont::new(Rgb888::YELLOW);
        //embedded_graphics::text::Text::new(self.current_time.as_str(), Point::new(3, 1), style).draw(target).ok();
        let style = AwtrixFont::new(if self.timer_started_at.is_some() { Rgb888::YELLOW } else { Rgb888::WHITE });

        for frame in self.icon.frames().skip(self.current_frame) {
            //frame.bounding_box().into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK)).draw(target).ok();
            frame.draw(target).ok();
            self.remaining_time_ms = frame.delay_centis as u32 * 10;
            break;
        }
        if self.current_frame != self.last_frame {
            self.last_frame = self.current_frame;
            self.current_frame_draw = Instant::now();
        } else if Instant::now().checked_duration_since(self.current_frame_draw).unwrap_or_default()
            > embassy_time::Duration::from_millis(self.remaining_time_ms as u64)
        {
            self.current_frame = (self.current_frame + 1) % self.icon.frames().count();
            //self.last_frame = usize::MAX;
        }

        if let Some(timer_started_at) = self.timer_started_at {
            let now = self.rtc.current_time_us();
            let now = chrono::NaiveDateTime::from_timestamp_micros(now as i64).unwrap();
            let elapsed = now - timer_started_at;
            let remaining = self.timer_duration - elapsed;
            if remaining.num_seconds() >= 0 {
                let minutes = remaining.num_hours();
                let seconds = remaining.num_minutes() % 60;
                write!(self.buf, "{:02}:{:02}", minutes, seconds).ok();
            } else {
                self.buf.push_str("00:00");
            }
        } else {
            write!(self.buf, "{:02}:{:02}", self.timer_duration.num_hours(), self.timer_duration.num_minutes() % 60)
                .ok();
        }

        embedded_graphics::text::Text::new(&self.buf.as_str(), Point::new(12, 1), style).draw(target).ok();
    }

    pub fn handle_event(&mut self, event: MatrixEventDetails) {
        info!("Timer page received event: {:?}", event);
        if event.is_single_press() {
            if event.has_left() {
                self.timer_duration -= chrono::Duration::minutes(1);
            }
            if event.has_right() {
                self.timer_duration += chrono::Duration::minutes(1);
            }
            if event.has_select() {
                if self.timer_started_at.is_none() {
                    let now = self.rtc.current_time_us();
                    let now = chrono::NaiveDateTime::from_timestamp_micros(now as i64).unwrap();
                    self.timer_started_at = Some(now);
                } else {
                    self.timer_started_at = None;
                }
            }
        }
    }
}
