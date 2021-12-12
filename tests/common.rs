#![cfg(test)]

use embedded_graphics::{
    geometry::{Dimensions, Point, Size},
    mock_display::MockDisplay,
    mono_font::{MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    text::Text,
    transform::Transform,
    Drawable,
};

const HELLO_WORLD: &str = "Hello World!";

fn baseline_point(font: &MonoFont) -> Point {
    Point::new(0, font.baseline as i32)
}

pub fn check_text_dimensions(font: &MonoFont) {
    let style = MonoTextStyle::new(font, BinaryColor::On);
    let hello = Text::new(HELLO_WORLD, Point::zero(), style);
    let empty = Text::new("", Point::zero(), style);

    assert_eq!(
        hello.bounding_box().size,
        Size::new(
            HELLO_WORLD.len() as u32 * font.character_size.width,
            font.character_size.height as u32
        )
    );
    assert_eq!(empty.bounding_box().size, Size::new(0, 0));
}

pub fn check_text_corners(font: &MonoFont) {
    let style = MonoTextStyle::new(font, BinaryColor::On);
    let hello = Text::new(HELLO_WORLD, baseline_point(font), style).translate(Point::new(5, -20));
    let empty = Text::new("", Point::zero(), style).translate(Point::new(10, 20));

    assert_eq!(hello.bounding_box().top_left, Point::new(5, -20));
    // Where e-g 0.6 calculated the bottom right point as top_left + size(),
    // 0.7's bounding box is a rectangle wich computes its bottom right point
    // as top_left + size() - Point::new(1, 1). So we need to take this
    // difference into account here.
    assert_eq!(
        hello.bounding_box().bottom_right(),
        Some(Point::new(
            (HELLO_WORLD.len() as u32 * font.character_size.width) as i32 + 5 - 1,
            (font.character_size.height as i32) - 20 - 1
        ))
    );
    assert_eq!(empty.bounding_box().top_left, Point::new(10, 20));
    assert_eq!(empty.bounding_box().bottom_right(), None);
}

pub fn check_correct_m(
    font: &MonoFont,
    reference: &[&str],
) -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(font, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new("Mm", baseline_point(font), style).draw(&mut display)?;

    assert_eq!(display, MockDisplay::from_pattern(reference));

    Ok(())
}

pub fn check_correct_ascii_borders(
    font: &MonoFont,
    reference: &[&str],
) -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(font, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new(" ~", baseline_point(font), style).draw(&mut display)?;

    assert_eq!(display, MockDisplay::from_pattern(reference));

    Ok(())
}

pub fn check_correct_dollar_y(
    font: &MonoFont,
    reference: &[&str],
) -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(font, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new("$y", baseline_point(font), style).draw(&mut display)?;

    assert_eq!(display, MockDisplay::from_pattern(reference));

    Ok(())
}

pub fn check_correct_latin1(
    font: &MonoFont,
    reference: &[&str],
) -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(font, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new("¡ÿ", baseline_point(font), style).draw(&mut display)?;

    assert_eq!(display, MockDisplay::from_pattern(reference));

    Ok(())
}

pub fn check_dont_panic(
    font: &MonoFont,
    one_replacement: &[&str],
    two_replacements: &[&str],
) -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(font, BinaryColor::On);

    let mut display = MockDisplay::new();
    Text::new("\0\r", baseline_point(font), style).draw(&mut display)?;
    // StyledTextIterator from e-g 0.6 implemented iterating over the lines
    // itself and only accepted '\n' as line ending. This resulted in "\0\r"
    // actually being rendered.
    //
    // With e-g 0.7, iterating over the lines is now done by str.lines() which
    // accepts both "\n" and "\r\n". This would give the same result as before
    // BUT the actual implementation is lenient with the last line, just
    // returns "\0" for it and we are going to accept this result.
    assert!("\0\r".lines().eq(["\0"]));
    assert_eq!(display, MockDisplay::from_pattern(one_replacement));

    let mut display = MockDisplay::new();
    Text::new("\x7F\u{A0}", baseline_point(font), style).draw(&mut display)?;
    assert_eq!(display, MockDisplay::from_pattern(two_replacements));

    let mut display = MockDisplay::new();
    Text::new("Ā💣", baseline_point(font), style).draw(&mut display)?;
    assert_eq!(display, MockDisplay::from_pattern(two_replacements));

    Ok(())
}
