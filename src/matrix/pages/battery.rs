use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use chrono::{Datelike, Timelike as _};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use num_traits::float::Float;

use crate::matrix::{
    fonts::AwtrixFont,
    pages::{PageTarget, Pages},
};

pub struct Battery {
    battery_level_percentage: f32,
    current_battery: String,
}

impl Battery {
    pub fn new() -> Pages {
        Pages::Battery(Box::new(Battery { battery_level_percentage: 0.0, current_battery: String::new() }))
    }

    pub fn update(&mut self) {
        self.current_battery.clear();
        self.battery_level_percentage = crate::adc::get_battery_level_percentage();
        write!(&mut self.current_battery, "{:.0}%", self.battery_level_percentage).ok();
    }

    pub fn render<T: PageTarget>(&self, target: &mut T) {
        target.clear(Rgb888::BLACK).ok();

        Rectangle::new(Point::new(0, 0), Size::new(5, 8))
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
            .draw(target)
            .ok();

        let rectangle =
            Rectangle::new(Point::new(0, 0), Size::new(1, 1)).into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK));
        rectangle.translate(Point::new(0, 0)).draw(target).ok();
        rectangle.translate(Point::new(4, 0)).draw(target).ok();

        let filled_height = (self.battery_level_percentage / 100.0 * 6.0).round() as i32;

        Rectangle::new(Point::new(1, 7 - filled_height), Size::new(3, filled_height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb888::MAGENTA))
            .draw(target)
            .ok();

        let time_style = AwtrixFont::new(Rgb888::YELLOW);
        Text::new(self.current_battery.as_str(), Point::new(12, 1), time_style).draw(target).ok();
    }
}
