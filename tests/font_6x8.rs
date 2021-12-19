#![cfg(test)]

mod common;
use embedded_vintage_fonts::FONT_6X8;

#[test]
fn text_dimensions() {
    common::check_text_dimensions(&FONT_6X8);
}

#[test]
fn text_corners() {
    common::check_text_corners(&FONT_6X8);
}

// FIXME: Port these test cases.
// #[test]
// fn correct_inverse_colored_m() -> Result<(), core::convert::Infallible> {
//     let mut display = MockDisplay::new();
//     let style = TextStyle {
//         font: Font6x8,
//         text_color: Some(BinaryColor::Off),
//         background_color: Some(BinaryColor::On),
//     };
//     Text::new("Mm", Point::zero())
//         .into_styled(style)
//         .draw(&mut display)?;
// 
//     assert_eq!(
//         display,
//         MockDisplay::from_pattern(&[
//             ".###.#######",
//             "..#..#######",
//             ".#.#.#..#.##",
//             ".#.#.#.#.#.#",
//             ".###.#.###.#",
//             ".###.#.###.#",
//             ".###.#.###.#",
//             "############",
//         ])
//     );
// 
//     Ok(())
// }
// 
// // tests if black on white has really the same behavior as white on black
// #[test]
// fn compare_inverse_colored_m() -> Result<(), core::convert::Infallible> {
//     let mut display_inverse = MockDisplay::new();
//     let style_inverse = TextStyle {
//         font: Font6x8,
//         text_color: Some(BinaryColor::Off),
//         background_color: Some(BinaryColor::On),
//     };
//     Text::new("Mm", Point::zero())
//         .into_styled(style_inverse)
//         .draw(&mut display_inverse)?;
// 
//     let mut display_normal = MockDisplay::new();
//     let style_normal = TextStyle {
//         font: Font6x8,
//         text_color: Some(BinaryColor::On),
//         background_color: Some(BinaryColor::Off),
//     };
//     Text::new("Mm", Point::zero())
//         .into_styled(style_normal)
//         .draw(&mut display_normal)?;
// 
//     for y in 0..display_inverse.height() {
//         for x in 0..display_inverse.width() {
//             let p = Point::new(x as i32, y as i32);
// 
//             let inverse_color = display_inverse.get_pixel(p);
//             let normal_color = display_normal.get_pixel(p);
// 
//             assert_eq!(inverse_color, normal_color.map(|c| c.invert()));
//         }
//     }
// 
//     Ok(())
// }


#[test]
fn correct_m() -> Result<(), core::convert::Infallible> {
    common::check_correct_m(
        &FONT_6X8,
        &[
            "#   #       ",
            "## ##       ",
            "# # # ## #  ",
            "# # # # # # ",
            "#   # #   # ",
            "#   # #   # ",
            "#   # #   # ",
            "            ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_ascii_borders() -> Result<(), core::convert::Infallible> {
    common::check_correct_ascii_borders(
        &FONT_6X8,
        &[
            "       ## # ",
            "      #  #  ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_dollar_y() -> Result<(), core::convert::Infallible> {
    common::check_correct_dollar_y(
        &FONT_6X8,
        &[
            "  #         ",
            " ####       ",
            "# #   #   # ",
            " ###  #   # ",
            "  # # #   # ",
            "####   #### ",
            "  #       # ",
            "       ###  ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_latin1() -> Result<(), core::convert::Infallible> {
    common::check_correct_latin1(
        &FONT_6X8,
        &[
            "  #    # #  ",
            "            ",
            "  #   #   # ",
            "  #   #   # ",
            "  #   #   # ",
            "  #    #### ",
            "  #       # ",
            "       ###  ",
            "            ",
        ],
    )?;

    Ok(())
}

#[test]
fn dont_panic() -> Result<(), core::convert::Infallible> {
    common::check_dont_panic(
        &FONT_6X8,
        &[
            " ###       ",
            "#   #      ",
            "    #      ",
            "   #       ",
            "  #        ",
            "           ",
            "  #        ",
        ],
        &[
            " ###   ### ",
            "#   # #   #",
            "    #     #",
            "   #     # ",
            "  #     #  ",
            "           ",
            "  #     #  ",
        ],
    )?;

    Ok(())
}

// FIXME: Port this test case.
// #[test]
// fn no_fill_doesnt_hang() -> Result<(), core::convert::Infallible> {
//     let mut display = MockDisplay::new();
//     Text::new(" ", Point::zero())
//         .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//         .draw(&mut display)?;
// 
//     assert_eq!(display, MockDisplay::new());
// 
//     Ok(())
// }
