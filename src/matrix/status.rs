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

use crate::{
    matrix::{
        fonts::AwtrixFont,
        pages::{PageTarget, Pages},
    },
    wifi::WiFiState,
};

pub struct Status {
    wifi_state: WiFiState,
}

impl Status {
    pub fn new() -> Self {
        Status { wifi_state: WiFiState::Disconnected }
    }

    pub fn update(&mut self) {
        self.wifi_state = crate::wifi::get_wifi_state();
    }

    pub fn render<T: PageTarget>(&self, target: &mut T) {
        let wifi_color = match self.wifi_state {
            WiFiState::Disconnected => Rgb888::RED,
            WiFiState::Scanning => Rgb888::YELLOW,
            WiFiState::Connecting => Rgb888::CSS_ORANGE,
            WiFiState::Connected => Rgb888::BLUE,
            WiFiState::Ip => Rgb888::GREEN,
        };

        target
            .draw_iter([
                Pixel(Point::new(30, 0), wifi_color),
                Pixel(Point::new(31, 0), wifi_color),
                Pixel(Point::new(31, 1), wifi_color),
            ])
            .ok();
    }
}
