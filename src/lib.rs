#![no_std]


use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
};


const GLYPH_MAPPING: StrGlyphMapping =
    StrGlyphMapping::new("\0\u{20}\u{7e}\0\u{a1}\u{ff}", '?' as usize - ' ' as usize);


pub const FONT_24X32: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font24x32_1bpp.raw"), 960),
    character_size: Size::new(24, 32),
    character_spacing: 0,
    baseline: 27,
    strikethrough: DecorationDimensions::new(14, 4),
    underline: DecorationDimensions::new(29, 4),
    glyph_mapping: &GLYPH_MAPPING,
};


pub const FONT_12X16: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font12x16_1bpp.raw"), 480),
    character_size: Size::new(12, 16),
    character_spacing: 0,
    baseline: 13,
    strikethrough: DecorationDimensions::new(7, 2),
    underline: DecorationDimensions::new(15, 2),
    glyph_mapping: &GLYPH_MAPPING,
};
