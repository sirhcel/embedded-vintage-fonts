#![cfg(test)]

use embedded_graphics::{
    geometry::{Dimensions, Point, Size},
    mock_display::MockDisplay,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    text::Text,
    transform::Transform,
    Drawable,
};
use embedded_vintage_fonts::FONT_24X32;

const BASELINE: usize = FONT_24X32.baseline as usize;
const BASELINE_POINT: Point = Point::new(0, BASELINE as i32);
const WIDTH: usize = FONT_24X32.character_size.width as usize;
const HEIGHT: usize = FONT_24X32.character_size.height as usize;
const HELLO_WORLD: &str = "Hello World!";

#[test]
fn text_dimensions() {
    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);
    let hello = Text::new(HELLO_WORLD, Point::zero(), style);
    let empty = Text::new("", Point::zero(), style);

    assert_eq!(
        hello.bounding_box().size,
        Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
    );
    assert_eq!(empty.bounding_box().size, Size::new(0, 0));
}

#[test]
fn text_corners() {
    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);
    let hello = Text::new(HELLO_WORLD, BASELINE_POINT, style)
        .translate(Point::new(5, -20));
    let empty = Text::new("", Point::zero(), style)
        .translate(Point::new(10, 20));

    assert_eq!(hello.bounding_box().top_left, Point::new(5, -20));
    // Where e-g 0.6 calculated the bottom right point as top_left + size(),
    // 0.7's bounding box is a rectangle wich computes its bottom right point
    // as top_left + size() - Point::new(1, 1). So we need to take this
    // difference into account here.
    assert_eq!(
        hello.bounding_box().bottom_right(),
        Some(Point::new(
            ((HELLO_WORLD.len() * WIDTH) as i32) + 5 - 1,
            (HEIGHT as i32) - 20 - 1
        ))
    );
    assert_eq!(empty.bounding_box().top_left, Point::new(10, 20));
    assert_eq!(empty.bounding_box().bottom_right(), None);
}

#[test]
fn correct_m() -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new("Mm", BASELINE_POINT, style)
        .draw(&mut display)?;

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "####            ####                          ",
            "####            ####                          ",
            "####            ####                          ",
            "####            ####                          ",
            "########    ########                          ",
            "########    ########                          ",
            "########    ########                          ",
            "########    ########                          ",
            "####    ####    ####    ########    ####      ",
            "####    ####    ####    ########    ####      ",
            "####    ####    ####    ########    ####      ",
            "####    ####    ####    ########    ####      ",
            "####    ####    ####    ####    ####    ####  ",
            "####    ####    ####    ####    ####    ####  ",
            "####    ####    ####    ####    ####    ####  ",
            "####    ####    ####    ####    ####    ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
            "####            ####    ####            ####  ",
        ])
    );

    Ok(())
}

#[test]
fn correct_ascii_borders() -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new(" ~", BASELINE_POINT, style)
        .draw(&mut display)?;

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "                            ########    #### ",
            "                            ########    #### ",
            "                            ########    #### ",
            "                            ########    #### ",
            "                        ####        ####     ",
            "                        ####        ####     ",
            "                        ####        ####     ",
            "                        ####        ####     ",
        ])
    );

    Ok(())
}

#[test]
fn correct_dollar_y() -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new("$y", BASELINE_POINT, style)
        .draw(&mut display)?;

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "        ####                                 ",
            "        ####                                 ",
            "        ####                                 ",
            "        ####                                 ",
            "    ################                         ",
            "    ################                         ",
            "    ################                         ",
            "    ################                         ",
            "####    ####            ####            #### ",
            "####    ####            ####            #### ",
            "####    ####            ####            #### ",
            "####    ####            ####            #### ",
            "    ############        ####            #### ",
            "    ############        ####            #### ",
            "    ############        ####            #### ",
            "    ############        ####            #### ",
            "        ####    ####    ####            #### ",
            "        ####    ####    ####            #### ",
            "        ####    ####    ####            #### ",
            "        ####    ####    ####            #### ",
            "################            ################ ",
            "################            ################ ",
            "################            ################ ",
            "################            ################ ",
            "        ####                            #### ",
            "        ####                            #### ",
            "        ####                            #### ",
            "        ####                            #### ",
            "                            ############     ",
            "                            ############     ",
            "                            ############     ",
            "                            ############     ",
        ])
    );

    Ok(())
}

#[test]
fn correct_latin1() -> Result<(), core::convert::Infallible> {
    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);
    let mut display = MockDisplay::new();
    Text::new("¡ÿ", BASELINE_POINT, style)
        .draw(&mut display)?;

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "        ####                ####    ####         ",
            "        ####                ####    ####         ",
            "        ####                ####    ####         ",
            "        ####                ####    ####         ",
            "                                                 ",
            "                                                 ",
            "                                                 ",
            "                                                 ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####            ####            ####     ",
            "        ####                ################     ",
            "        ####                ################     ",
            "        ####                ################     ",
            "        ####                ################     ",
            "        ####                            ####     ",
            "        ####                            ####     ",
            "        ####                            ####     ",
            "        ####                            ####     ",
            "                            ############         ",
            "                            ############         ",
            "                            ############         ",
            "                            ############         ",
        ])
    );

    Ok(())
}

#[test]
fn dont_panic() -> Result<(), core::convert::Infallible> {
    let one_question_mark = MockDisplay::from_pattern(&[
        "    ############                            ",
        "    ############                            ",
        "    ############                            ",
        "    ############                            ",
        "####            ####                        ",
        "####            ####                        ",
        "####            ####                        ",
        "####            ####                        ",
        "                ####                        ",
        "                ####                        ",
        "                ####                        ",
        "                ####                        ",
        "            ####                            ",
        "            ####                            ",
        "            ####                            ",
        "            ####                            ",
        "        ####                                ",
        "        ####                                ",
        "        ####                                ",
        "        ####                                ",
        "                                            ",
        "                                            ",
        "                                            ",
        "                                            ",
        "        ####                                ",
        "        ####                                ",
        "        ####                                ",
        "        ####                                ",
    ]);
    let two_question_marks = MockDisplay::from_pattern(&[
        "    ############            ############     ",
        "    ############            ############     ",
        "    ############            ############     ",
        "    ############            ############     ",
        "####            ####    ####            #### ",
        "####            ####    ####            #### ",
        "####            ####    ####            #### ",
        "####            ####    ####            #### ",
        "                ####                    #### ",
        "                ####                    #### ",
        "                ####                    #### ",
        "                ####                    #### ",
        "            ####                    ####     ",
        "            ####                    ####     ",
        "            ####                    ####     ",
        "            ####                    ####     ",
        "        ####                    ####         ",
        "        ####                    ####         ",
        "        ####                    ####         ",
        "        ####                    ####         ",
        "                                             ",
        "                                             ",
        "                                             ",
        "                                             ",
        "        ####                    ####         ",
        "        ####                    ####         ",
        "        ####                    ####         ",
        "        ####                    ####         ",
    ]);

    let style = MonoTextStyle::new(&FONT_24X32, BinaryColor::On);

    let mut display = MockDisplay::new();
    Text::new("\0\r", BASELINE_POINT, style)
        .draw(&mut display)?;
    // StyledTextIterator from e-g 0.6 implemented iterating over the lines
    // itself and only accepted '\n' as line ending. This resulted in "\0\r"
    // actually being rendered.
    //
    // With e-g 0.7, iterating over the lines is now done by str.lines() which
    // accepts both "\n" and "\r\n". This would give the same result as before
    // BUT the actual implementation is lenient with the last line, just
    // returns "\0" for it and we are going to accept this result.
    assert!("\0\r".lines().eq(["\0"]));
    assert_eq!(display, one_question_mark);

    let mut display = MockDisplay::new();
    Text::new("\x7F\u{A0}", BASELINE_POINT, style)
        .draw(&mut display)?;
    assert_eq!(display, two_question_marks);

    let mut display = MockDisplay::new();
    Text::new("Ā💣", BASELINE_POINT, style)
        .draw(&mut display)?;
    assert_eq!(display, two_question_marks);

    Ok(())
}
