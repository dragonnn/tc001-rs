use alloc::boxed::Box;

use embedded_graphics::{pixelcolor::Rgb888, prelude::DrawTarget};

pub trait PageTarget: DrawTarget<Color = Rgb888, Error = core::convert::Infallible> {}

impl PageTarget
    for smart_leds_matrix::SmartLedMatrix<
        esp_hal_smartled::SmartLedsAdapter<'_, 6400>,
        smart_leds_matrix::layout::Rectangular<smart_leds_matrix::layout::invert_axis::Tc001>,
        256,
    >
{
}

pub trait Page {
    fn update(&mut self);
}

pub trait PageRender: Page {
    fn render<T: PageTarget>(&self, target: &mut T);
}
