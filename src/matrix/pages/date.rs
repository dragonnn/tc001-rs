use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use crate::matrix::{fonts::AwtrixFont, pages::Pages};

pub struct Date {
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
    current_time: String,
}

impl Date {
    pub fn new(rtc: &'static esp_hal::rtc_cntl::Rtc<'static>) -> Pages {
        Pages::Date(Box::new(Date { rtc, current_time: String::from("00:00:00") }))
    }

    pub fn update(&mut self) {
        self.current_time.clear();
        let now = self.rtc.current_time_us();
        let now = chrono::NaiveDateTime::from_timestamp_micros(now as i64).unwrap();
        let date = now.date();
        write!(&mut self.current_time, "{}", date.format("%d.%m.%y")).ok();
    }

    pub fn render<T: super::PageTarget>(&self, target: &mut T) {
        embedded_graphics::primitives::Rectangle::new(Point::new(0, 0), Size::new(32, 8))
            .into_styled(embedded_graphics::primitives::PrimitiveStyle::with_fill(
                embedded_graphics::pixelcolor::Rgb888::BLACK,
            ))
            .draw(target)
            .ok();
        //let font = embedded_graphics::mono_font::ascii::FONT_4X6;
        //let style = embedded_graphics::mono_font::MonoTextStyle::new(&font, embedded_graphics::pixelcolor::Rgb888::RED);
        //let style = super::font::mateine(embedded_graphics::pixelcolor::Rgb888::RED);
        let style = AwtrixFont::new(Rgb888::RED);
        embedded_graphics::text::Text::new(self.current_time.as_str(), Point::new(0, 0), style).draw(target).ok();
    }
}
