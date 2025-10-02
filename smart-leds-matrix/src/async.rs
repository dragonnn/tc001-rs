use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
    Pixel,
};

use crate::layout::Layout;
use smart_leds::{brightness, gamma, SmartLedsWriteAsync, RGB8};

/// The wrapper for the LED driver.
///
/// This receives the `SmartLedsWriter` trait implementations along with a
/// `Transformation` that describes the pixels mapping between the LED
/// strip placement and the matrix's x y coordinates.
pub struct SmartLedMatrixAsync<T, L, const N: usize> {
    writer: T,
    layout: L,
    content: [RGB8; N],
    brightness: u8,
}

impl<T, L, const N: usize> SmartLedMatrixAsync<T, L, N> {
    pub fn set_brightness(&mut self, new_brightness: u8) {
        self.brightness = new_brightness;
    }

    pub fn brightness(&self) -> u8 {
        self.brightness
    }
}

impl<T: SmartLedsWriteAsync, L: Layout, const N: usize> SmartLedMatrixAsync<T, L, N>
where
    <T as SmartLedsWriteAsync>::Color: From<RGB8>,
{
    pub fn new(writer: T, layout: L) -> Self {
        Self {
            writer,
            layout,
            content: [RGB8::default(); N],
            brightness: 255,
        }
    }

    pub async fn flush(&mut self) -> Result<(), T::Error> {
        let iter = brightness(self.content.as_slice().iter().cloned(), self.brightness);
        self.writer.write(iter).await
    }
    pub async fn flush_with_gamma(&mut self) -> Result<(), T::Error> {
        let iter = brightness(
            gamma(self.content.as_slice().iter().cloned()),
            self.brightness,
        );
        self.writer.write(iter).await
    }
}

impl<T: SmartLedsWriteAsync, L: Layout, const N: usize> DrawTarget for SmartLedMatrixAsync<T, L, N>
where
    <T as SmartLedsWriteAsync>::Color: From<RGB8>,
{
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb888>>,
    {
        for Pixel(pos, color) in pixels {
            if let Some(t) = self
                .layout
                .map(pos)
                .and_then(|index| self.content.get_mut(index))
            {
                *t = RGB8::new(color.r(), color.g(), color.b());
            }
        }

        Ok(())
    }
}

impl<T, L: Layout, const N: usize> OriginDimensions for SmartLedMatrixAsync<T, L, N> {
    fn size(&self) -> Size {
        self.layout.size()
    }
}
