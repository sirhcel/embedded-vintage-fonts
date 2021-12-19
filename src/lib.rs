#![no_std]

use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
};

const GLYPH_MAPPING: StrGlyphMapping =
    StrGlyphMapping::new("\0\u{20}\u{7e}\0\u{a1}\u{ff}", '?' as usize - ' ' as usize);
const GLYPH_MAPPING_6X12: StrGlyphMapping =
    StrGlyphMapping::new("\0\u{20}\u{7e}", '?' as usize - ' ' as usize);

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

pub const FONT_8X16: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font8x16_1bpp.raw"), 240),
    character_size: Size::new(8, 16),
    character_spacing: 0,
    baseline: 11,
    strikethrough: DecorationDimensions::new(6, 2),
    underline: DecorationDimensions::new(13, 2),
    glyph_mapping: &GLYPH_MAPPING,
};

pub const FONT_6X12: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font6x12_1bpp.raw"), 96),
    character_size: Size::new(6, 12),
    character_spacing: 0,
    baseline: 9,
    strikethrough: DecorationDimensions::new(5, 1),
    underline: DecorationDimensions::new(11, 1),
    glyph_mapping: &GLYPH_MAPPING_6X12,
};

pub const FONT_6X8: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font6x8_1bpp.raw"), 240),
    character_size: Size::new(6, 8),
    character_spacing: 0,
    baseline: 6,
    strikethrough: DecorationDimensions::new(3, 1),
    underline: DecorationDimensions::new(8, 1),
    glyph_mapping: &GLYPH_MAPPING,
};
