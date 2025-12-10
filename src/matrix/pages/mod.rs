use alloc::{boxed::Box, string::String};

use embedded_graphics::{pixelcolor::Rgb888, prelude::DrawTarget};

mod date;
mod time;

pub use date::Date;
pub use time::Time;

pub trait PageTarget: DrawTarget<Color = Rgb888, Error = core::convert::Infallible> {}

impl<T: DrawTarget<Color = Rgb888, Error = core::convert::Infallible>> PageTarget for T {}

pub enum Pages {
    Time(Box<time::Time>),
    Date(Box<date::Date>),
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
