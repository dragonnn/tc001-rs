use alloc::{boxed::Box, string::String};

use embedded_graphics::{pixelcolor::Rgb888, prelude::DrawTarget};

mod battery;
mod date;
mod time;
mod timer;

pub use battery::Battery;
pub use date::Date;
pub use time::Time;
pub use timer::Timer;

use crate::matrix::event::MatrixEventDetails;

pub trait PageTarget: DrawTarget<Color = Rgb888, Error = core::convert::Infallible> {}

impl<T: DrawTarget<Color = Rgb888, Error = core::convert::Infallible>> PageTarget for T {}

pub enum Pages {
    Time(Box<time::Time>),
    Date(Box<date::Date>),
    Timer(Box<timer::Timer>),
    Battery(Box<battery::Battery>),
}

impl Pages {
    pub fn update(&mut self) {
        match self {
            Pages::Time(page) => page.update(),
            Pages::Date(page) => page.update(),
            Pages::Timer(page) => page.update(),
            Pages::Battery(page) => page.update(),
        }
    }

    pub fn render<T: PageTarget>(&self, target: &mut T) {
        match self {
            Pages::Time(page) => page.render(target),
            Pages::Date(page) => page.render(target),
            Pages::Timer(page) => page.render(target),
            Pages::Battery(page) => page.render(target),
        }
    }

    pub fn idle_update(&mut self) {
        self.update();
    }

    pub fn handle_event(&mut self, event: MatrixEventDetails) {
        match self {
            Pages::Time(page) => page.handle_event(event),
            Pages::Date(page) => page.handle_event(event),
            Pages::Timer(page) => page.handle_event(event),
            Pages::Battery(page) => page.handle_event(event),
        }
    }
}
