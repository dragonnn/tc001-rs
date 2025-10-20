use embedded_graphics::pixelcolor::Rgb888;
use embedded_ttf::{FontTextStyle, FontTextStyleBuilder};

static MATEINE: &[u8] = include_bytes!("../../mateine.ttf");

pub fn mateine(color: Rgb888) -> embedded_ttf::FontTextStyle<Rgb888> {
    FontTextStyleBuilder::new(rusttype::Font::try_from_bytes(MATEINE).unwrap()).font_size(7).text_color(color).build()
}
