use alloc::{boxed::Box, string::String};

use embedded_graphics::{pixelcolor::Rgb888, prelude::DrawTarget};

pub trait PageTarget: DrawTarget<Color = Rgb888, Error = core::convert::Infallible> {}

impl<T: DrawTarget<Color = Rgb888, Error = core::convert::Infallible>> PageTarget for T {}

pub enum Pages {
    Time(Box<super::time::Time>),
    Date(Box<super::date::Date>),
}

impl Pages {
    pub fn update(&mut self) {
        match self {
            Pages::Time(page) => page.update(),
            Pages::Date(page) => page.update(),
        }
    }

    pub fn render<T: PageTarget>(&self, target: &mut T) {
        match self {
            Pages::Time(page) => page.render(target),
            Pages::Date(page) => page.render(target),
        }
    }
}
