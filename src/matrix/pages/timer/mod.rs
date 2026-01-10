use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use embassy_time::Instant;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use crate::matrix::{event::MatrixEventDetails, fonts::AwtrixFont, pages::Pages};

static POMODOR_ICON: &[u8] = include_bytes!("./pomodoro.gif");

pub struct Timer {
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
    icon: tinygif::Gif<'static>,
    last_frame: usize,
    current_frame: usize,
    current_frame_draw: Instant,
    remaining_time_ms: u32,
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
        }))
    }

    pub fn update(&mut self) {}

    pub fn render<T: super::PageTarget>(&mut self, target: &mut T) {
        //
        //let font = embedded_graphics::mono_font::ascii::FONT_4X6;
        //let style = embedded_graphics::mono_font::MonoTextStyle::new(&font, embedded_graphics::pixelcolor::Rgb888::RED);
        //let style = super::font::mateine(embedded_graphics::pixelcolor::Rgb888::RED);
        //let style = AwtrixFont::new(Rgb888::YELLOW);
        //embedded_graphics::text::Text::new(self.current_time.as_str(), Point::new(3, 1), style).draw(target).ok();
        if self.current_frame != self.last_frame {
            target.clear(Rgb888::BLACK).ok();
            for frame in self.icon.frames().skip(self.current_frame) {
                frame.draw(target).ok();
                self.remaining_time_ms = frame.delay_centis as u32 * 10;
                break;
            }
            self.last_frame = self.current_frame;
            self.current_frame_draw = Instant::now();
        } else if Instant::now().checked_duration_since(self.current_frame_draw).unwrap_or_default()
            > embassy_time::Duration::from_millis(self.remaining_time_ms as u64)
        {
            self.current_frame = (self.current_frame + 1) % self.icon.frames().count();
            //self.last_frame = usize::MAX;
        }
    }

    pub fn handle_event(&mut self, event: MatrixEventDetails) {
        info!("Timer page received event: {:?}", event);
    }
}
