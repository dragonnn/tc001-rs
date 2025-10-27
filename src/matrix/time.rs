use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use chrono::Datelike;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

use super::page::Pages;

pub struct Time {
    rtc: &'static esp_hal::rtc_cntl::Rtc<'static>,
    current_time: String,
    current_day: String,
    current_day_of_week: u8,
}

impl Time {
    pub fn new(rtc: &'static esp_hal::rtc_cntl::Rtc<'static>) -> Pages {
        Pages::Time(Box::new(Time {
            rtc,
            current_time: String::from("00:00:00"),
            current_day: String::from("00"),
            current_day_of_week: 0,
        }))
    }

    pub fn update(&mut self) {
        self.current_time.clear();
        self.current_day.clear();
        let now = self.rtc.current_time_us();
        let now = chrono::NaiveDateTime::from_timestamp_micros(now as i64).unwrap();
        write!(&mut self.current_time, "{}", now.time().format("%H:%M")).ok();
        write!(&mut self.current_day, "{}", now.date().format("%d")).ok();
        self.current_day_of_week = now.date().weekday().number_from_monday() as u8 - 1;
    }

    pub fn render<T: super::page::PageTarget>(&self, target: &mut T) {
        Rectangle::new(Point::new(0, 0), Size::new(32, 8))
            .into_styled(PrimitiveStyle::with_fill(embedded_graphics::pixelcolor::Rgb888::BLACK))
            .draw(target)
            .ok();
        Rectangle::new(Point::new(0, 0), Size::new(9, 2))
            .into_styled(PrimitiveStyle::with_fill(embedded_graphics::pixelcolor::Rgb888::RED))
            .draw(target)
            .ok();
        Rectangle::new(Point::new(0, 2), Size::new(9, 6))
            .into_styled(PrimitiveStyle::with_fill(embedded_graphics::pixelcolor::Rgb888::WHITE))
            .draw(target)
            .ok();

        let day_style = super::awtrix::AwtrixFont::new(Rgb888::BLACK);
        Text::new(self.current_day.as_str(), Point::new(1, 2), day_style).draw(target).ok();
        let time_style = super::awtrix::AwtrixFont::new(Rgb888::YELLOW);
        Text::new(self.current_time.as_str(), Point::new(12, 1), time_style).draw(target).ok();
        for i in 0..7 {
            let color = if i == self.current_day_of_week { Rgb888::WHITE } else { Rgb888::CSS_GRAY };
            let i = i as i32;
            Rectangle::new(Point::new(10 + (2 * i + i), 7), Size::new(2, 1))
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(target)
                .ok();
        }
    }
}
