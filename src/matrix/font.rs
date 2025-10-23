use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor as _, text::renderer::TextRenderer};
use embedded_ttf::{FontTextStyle, FontTextStyleBuilder};

static MATEINE: &[u8] = include_bytes!("../../my 3x5 tiny mono pixel font.ttf");
//static MATEINE: &[u8] = include_bytes!("../../mateine.ttf");

pub fn mateine(color: Rgb888) -> embedded_ttf::FontTextStyle<Rgb888> {
    let font = rusttype::Font::try_from_bytes(MATEINE).unwrap();

    let dot = font.glyph('.');

    FontTextStyleBuilder::new(font)
        .font_size(6)
        .text_color(color)
        //.background_color(embedded_graphics::pixelcolor::Rgb888::WHITE)
        .build()
}

pub struct AwrtixFont;
impl TextRenderer for AwrtixFont {
    type Color = Rgb888;

    fn draw_string<D>(
        &self,
        text: &str,
        position: embedded_graphics::prelude::Point,
        baseline: embedded_graphics::text::Baseline,
        target: &mut D,
    ) -> Result<embedded_graphics::prelude::Point, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        todo!()
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: embedded_graphics::prelude::Point,
        baseline: embedded_graphics::text::Baseline,
        target: &mut D,
    ) -> Result<embedded_graphics::prelude::Point, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        todo!()
    }

    fn measure_string(
        &self,
        text: &str,
        position: embedded_graphics::prelude::Point,
        baseline: embedded_graphics::text::Baseline,
    ) -> embedded_graphics::text::renderer::TextMetrics {
        todo!()
    }

    fn line_height(&self) -> u32 {
        todo!()
    }
}
