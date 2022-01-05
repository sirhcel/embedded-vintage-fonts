#![no_std]

//! The monospace fonts from
//! [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics/tree/embedded-graphics-v0.6.2)
//! back the overhauled font handling form 0.7. The fronts provided here use
//! the same bitmaps with new
//! [`MonoFont`](embedded_graphics::mono_font::MonoFont) structs.
//!
//! # Example
//!
//! Draw the text "Hello!" to a mock display using the 6 x 8 pixel font
//! [`FONT_6X8`].
//!
//! ```rust
//! use embedded_graphics::{
//!     mock_display::MockDisplay,
//!     mono_font::MonoTextStyle,
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::Text,
//! };
//! use embedded_vintage_fonts::FONT_6X8;
//!
//! # fn main() -> Result<(), core::convert::Infallible> {
//! let mut display = MockDisplay::new();
//! let style = MonoTextStyle::new(&FONT_6X8, BinaryColor::On);
//!
//! Text::new("Hello!", Point::new(0, FONT_6X8.baseline as i32), style)
//!     .draw(&mut display)?;
//!
//! assert_eq!(
//!     display,
//!     MockDisplay::from_pattern(&[
//!         "#   #        ##    ##           #",
//!         "#   #         #     #           #",
//!         "#   #  ###    #     #    ###    #",
//!         "##### #   #   #     #   #   #   #",
//!         "#   # #####   #     #   #   #   #",
//!         "#   # #       #     #   #   #    ",
//!         "#   #  ###   ###   ###   ###    #",
//!     ])
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Glyph Coverage
//!
//! Most fonts from this crate provide support for [ISO/IEC
//! 8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1) (Latin-1).
//! [`FONT_6X12`] supports only [ASCII](https://en.wikipedia.org/wiki/ASCII)
//! characters.

use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
};

const GLYPH_MAPPING: StrGlyphMapping =
    StrGlyphMapping::new("\0\u{20}\u{7e}\0\u{a1}\u{ff}", '?' as usize - ' ' as usize);
const GLYPH_MAPPING_6X12: StrGlyphMapping =
    StrGlyphMapping::new("\0\u{20}\u{7e}", '?' as usize - ' ' as usize);

/// An upscaled version of [`FONT_12X16`] previously known as `Font24x32`.
pub const FONT_24X32: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font24x32_1bpp.raw"), 960),
    character_size: Size::new(24, 32),
    character_spacing: 0,
    baseline: 27,
    strikethrough: DecorationDimensions::new(14, 4),
    underline: DecorationDimensions::new(29, 4),
    glyph_mapping: &GLYPH_MAPPING,
};

/// The 12 x 16 pixel font formerly known `Font12x16`.
///
/// ![Source
/// image](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font12x16.png)
pub const FONT_12X16: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font12x16_1bpp.raw"), 480),
    character_size: Size::new(12, 16),
    character_spacing: 0,
    baseline: 13,
    strikethrough: DecorationDimensions::new(7, 2),
    underline: DecorationDimensions::new(15, 2),
    glyph_mapping: &GLYPH_MAPPING,
};

/// The 8 x 16 pixel font formerly known `Font8x16`.
///
/// ![Source
/// image](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font8x16.png)
pub const FONT_8X16: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font8x16_1bpp.raw"), 240),
    character_size: Size::new(8, 16),
    character_spacing: 0,
    baseline: 11,
    strikethrough: DecorationDimensions::new(6, 2),
    underline: DecorationDimensions::new(13, 2),
    glyph_mapping: &GLYPH_MAPPING,
};

/// The 6 x 12 pixel font formerly known `Font6x12`.
///
/// ![Source
/// image](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font6x12.png)
pub const FONT_6X12: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font6x12_1bpp.raw"), 96),
    character_size: Size::new(6, 12),
    character_spacing: 0,
    baseline: 9,
    strikethrough: DecorationDimensions::new(5, 1),
    underline: DecorationDimensions::new(11, 1),
    glyph_mapping: &GLYPH_MAPPING_6X12,
};

/// The 6 x 8 pixel font formerly known `Font6x8`.
///
/// ![Source
/// image](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font6x8.png)
pub const FONT_6X8: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("../data/font6x8_1bpp.raw"), 240),
    character_size: Size::new(6, 8),
    character_spacing: 0,
    baseline: 6,
    strikethrough: DecorationDimensions::new(3, 1),
    underline: DecorationDimensions::new(8, 1),
    glyph_mapping: &GLYPH_MAPPING,
};
