#![cfg(test)]

mod common;
use embedded_graphics::{
    geometry::Point,
    mock_display::MockDisplay,
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    text::Text,
    transform::Transform,
    Drawable,
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

#[test]
fn negative_y_no_infinite_loop() -> Result<(), core::convert::Infallible> {
    let font = &FONT_6X12;
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    let mut display = MockDisplay::new();
    display.set_allow_out_of_bounds_drawing(true);

    let mut text = Text::new("Testing string", common::baseline_point(font), style);
    text.translate_mut(Point::new(0, -12));
    text.draw(&mut display)?;

    // Draing is expected to take place outside the display bounds.
    //
    // In embedded-graphics 0.6 there was a pixel iterator from StyledText
    // available so we could check for the actual number of affected pixels.
    // But this is currently not the case for 0.7.
    assert!(display.affected_area().is_zero_sized());

    Ok(())
}

#[test]
fn negative_x_no_infinite_loop() -> Result<(), core::convert::Infallible> {
    let font = &FONT_6X12;
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    let mut display = MockDisplay::new();
    display.set_allow_out_of_bounds_drawing(true);

    let mut text = Text::new("A", common::baseline_point(font), style);
    text.translate_mut(Point::new(-6, 0));
    text.draw(&mut display)?;

    // Draing is expected to take place outside the display bounds.
    //
    // In embedded-graphics 0.6 there was a pixel iterator from StyledText
    // available so we could check for the actual number of affected pixels.
    // But this is currently not the case for 0.7.
    assert!(display.affected_area().is_zero_sized());

    Ok(())
}
