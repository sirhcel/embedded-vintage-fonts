#![cfg(test)]

use super::*;
use crate::{
    drawable::Drawable,
    fonts::{Font, Text},
    geometry::{Dimensions, Point, Size},
    mock_display::MockDisplay,
    pixelcolor::BinaryColor,
    style::TextStyle,
    transform::Transform,
};

const WIDTH: usize = Font24x32::CHARACTER_SIZE.width as usize;
const HEIGHT: usize = Font24x32::CHARACTER_SIZE.height as usize;
const HELLO_WORLD: &'static str = "Hello World!";

#[test]
fn text_dimensions() {
    let style = TextStyle::new(Font24x32, BinaryColor::On);
    let hello = Text::new(HELLO_WORLD, Point::zero()).into_styled(style);
    let empty = Text::new("", Point::zero()).into_styled(style);

    assert_eq!(
        hello.size(),
        Size::new((HELLO_WORLD.len() * WIDTH) as u32, HEIGHT as u32)
    );
    assert_eq!(empty.size(), Size::new(0, 0));
}

#[test]
fn text_corners() {
    let style = TextStyle::new(Font24x32, BinaryColor::On);
    let hello = Text::new(HELLO_WORLD, Point::zero())
        .into_styled(style)
        .translate(Point::new(5, -20));
    let empty = Text::new("", Point::zero())
        .into_styled(style)
        .translate(Point::new(10, 20));

    assert_eq!(hello.top_left(), Point::new(5, -20));
    assert_eq!(
        hello.bottom_right(),
        Point::new(
            ((HELLO_WORLD.len() * WIDTH) as i32) + 5,
            (HEIGHT as i32) - 20
        )
    );
    assert_eq!(empty.top_left(), Point::new(10, 20));
    assert_eq!(empty.bottom_right(), Point::new(10, 20));
}

#[test]
fn correct_m() -> Result<(), core::convert::Infallible> {
    let mut display = MockDisplay::new();
    Text::new("Mm", Point::zero())
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
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
    let mut display = MockDisplay::new();
    Text::new(" ~", Point::zero())
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
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
    let mut display = MockDisplay::new();
    Text::new("$y", Point::zero())
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
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
    let mut display = MockDisplay::new();
    Text::new("¡ÿ", Point::zero())
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
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

    let style = TextStyle::new(Font24x32, BinaryColor::On);

    let mut display = MockDisplay::new();
    Text::new("\0\r", Point::zero())
        .into_styled(style)
        .draw(&mut display)?;
    assert_eq!(display, two_question_marks);

    let mut display = MockDisplay::new();
    Text::new("\x7F\u{A0}", Point::zero())
        .into_styled(style)
        .draw(&mut display)?;
    assert_eq!(display, two_question_marks);

    let mut display = MockDisplay::new();
    Text::new("Ā💣", Point::zero())
        .into_styled(style)
        .draw(&mut display)?;
    assert_eq!(display, two_question_marks);

    Ok(())
}
