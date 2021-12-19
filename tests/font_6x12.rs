#![cfg(test)]

mod common;
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    text::Text,
    transform::Transform,
};
use embedded_vintage_fonts::FONT_6X12;

#[test]
fn text_dimensions() {
    common::check_text_dimensions(&FONT_6X12);
}

#[test]
fn text_corners() {
    common::check_text_corners(&FONT_6X12);
}

#[test]
fn correct_m() -> Result<(), core::convert::Infallible> {
    common::check_correct_m(
        &FONT_6X12,
        &[
            "            ",
            "#   #       ",
            "## ##       ",
            "## ##       ",
            "# # # ####  ",
            "# # # # # # ",
            "#   # # # # ",
            "#   # # # # ",
            "#   # # # # ",
            "#   # # # # ",
            "            ",
            "            ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_ascii_borders() -> Result<(), core::convert::Infallible> {
    common::check_correct_ascii_borders(
        &FONT_6X12,
        &[
            "        # # ",
            "       #### ",
            "       # #  ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_dollar_y() -> Result<(), core::convert::Infallible> {
    common::check_correct_dollar_y(
        &FONT_6X12,
        &[
            "            ",
            "  #         ",
            " ###        ",
            "# # #       ",
            "# #    #  # ",
            " ###   #  # ",
            "  # #  #  # ",
            "  # #  #  # ",
            "# # #  #  # ",
            " ###    ### ",
            "  #       # ",
            "        ##  ",
        ],
    )?;

    Ok(())
}

// This font only supports only ASCII and so there is no test case for Latin 1.

#[test]
fn dont_panic() -> Result<(), core::convert::Infallible> {
    common::check_dont_panic(
        &FONT_6X12,
        &[
            "            ",
            "  ##        ",
            " #  #       ",
            " #  #       ",
            "    #       ",
            "   #        ",
            "  #         ",
            "  #         ",
            "            ",
            "  #         ",
            "            ",
            "            ",
        ],
        &[
            "            ",
            "  ##    ##  ",
            " #  #  #  # ",
            " #  #  #  # ",
            "    #     # ",
            "   #     #  ",
            "  #     #   ",
            "  #     #   ",
            "            ",
            "  #     #   ",
            "            ",
            "            ",
        ],
    )?;

    Ok(())
}

// FIXME: Port these test cases.
// #[test]
// fn negative_y_no_infinite_loop() {
//     let font = &FONT_6X12;
//     let style = MonoTextStyleBuilder::new()
//         .font(font)
//         .text_color(BinaryColor::On)
//         .background_color(BinaryColor::Off)
//         .build();
// 
//     let mut text = Text::new("Testing string", common::baseline_point(font), style);
//     text.translate_mut(Point::new(0, -12));
// 
//     assert_eq!(text.pixels().count(), 6 * 12 * "Testing string".len());
// }
// 
// #[test]
// fn negative_x_no_infinite_loop() {
//     let style = TextStyle {
//         font: Font6x12,
//         text_color: Some(BinaryColor::On),
//         background_color: Some(BinaryColor::Off),
//     };
// 
//     let mut text = Text::new("A", Point::zero()).into_styled(style);
//     text.translate_mut(Point::new(-6, 0));
// 
//     assert_eq!(text.into_iter().count(), 6 * 12);
// }
