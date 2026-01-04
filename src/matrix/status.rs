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
    ha::HaState,
    matrix::{
        fonts::AwtrixFont,
        pages::{PageTarget, Pages},
    },
    wifi::WiFiState,
};

pub struct Status {
    wifi_state: WiFiState,
    ha_state: HaState,
}

impl Status {
    pub fn new() -> Self {
        Status { wifi_state: WiFiState::Disconnected, ha_state: HaState::Disconnected }
    }

    pub fn update(&mut self) {
        self.wifi_state = crate::wifi::get_wifi_state();
        self.ha_state = crate::ha::get_ha_state();
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

        let ha_color = match self.ha_state {
            HaState::Disconnected => Rgb888::RED,
            HaState::TransportConnecting => Rgb888::YELLOW,
            HaState::TransportConnected => Rgb888::CSS_ORANGE,
            HaState::MqttConnecting => Rgb888::CSS_ORANGE,
            HaState::MqttConnected => Rgb888::GREEN,
        };

        target
            .draw_iter([
                Pixel(Point::new(30, 7), ha_color),
                Pixel(Point::new(31, 7), ha_color),
                Pixel(Point::new(31, 6), ha_color),
            ])
            .ok();
    }
}
