#![cfg(test)]

mod common;
use embedded_vintage_fonts::FONT_12X16;

#[test]
fn text_dimensions() {
    common::check_text_dimensions(&FONT_12X16);
}

#[test]
fn text_corners() {
    common::check_text_corners(&FONT_12X16);
}

#[test]
fn correct_m() -> Result<(), core::convert::Infallible> {
    common::check_correct_m(
        &FONT_12X16,
        &[
            "##      ##              ",
            "##      ##              ",
            "####  ####              ",
            "####  ####              ",
            "##  ##  ##  ####  ##    ",
            "##  ##  ##  ####  ##    ",
            "##  ##  ##  ##  ##  ##  ",
            "##  ##  ##  ##  ##  ##  ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_ascii_borders() -> Result<(), core::convert::Infallible> {
    common::check_correct_ascii_borders(
        &FONT_12X16,
        &[
            "              ####  ##  ",
            "              ####  ##  ",
            "            ##    ##    ",
            "            ##    ##    ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_dollar_y() -> Result<(), core::convert::Infallible> {
    common::check_correct_dollar_y(
        &FONT_12X16,
        &[
            "    ##                  ",
            "    ##                  ",
            "  ########              ",
            "  ########              ",
            "##  ##      ##      ##  ",
            "##  ##      ##      ##  ",
            "  ######    ##      ##  ",
            "  ######    ##      ##  ",
            "    ##  ##  ##      ##  ",
            "    ##  ##  ##      ##  ",
            "########      ########  ",
            "########      ########  ",
            "    ##              ##  ",
            "    ##              ##  ",
            "              ######    ",
            "              ######    ",
        ],
    )?;

    Ok(())
}

#[test]
fn correct_latin1() -> Result<(), core::convert::Infallible> {
    common::check_correct_latin1(
        &FONT_12X16,
        &[
            "    ##        ##  ##    ",
            "    ##        ##  ##    ",
            "                        ",
            "                        ",
            "    ##      ##      ##  ",
            "    ##      ##      ##  ",
            "    ##      ##      ##  ",
            "    ##      ##      ##  ",
            "    ##      ##      ##  ",
            "    ##      ##      ##  ",
            "    ##        ########  ",
            "    ##        ########  ",
            "    ##              ##  ",
            "    ##              ##  ",
            "              ######    ",
            "              ######    ",
        ],
    )?;

    Ok(())
}

#[test]
fn dont_panic() -> Result<(), core::convert::Infallible> {
    common::check_dont_panic(
        &FONT_12X16,
        &[
            "  ######                ",
            "  ######                ",
            "##      ##              ",
            "##      ##              ",
            "        ##              ",
            "        ##              ",
            "      ##                ",
            "      ##                ",
            "    ##                  ",
            "    ##                  ",
            "                        ",
            "                        ",
            "    ##                  ",
            "    ##                  ",
            "                        ",
            "                        ",
        ],
        &[
            "  ######      ######    ",
            "  ######      ######    ",
            "##      ##  ##      ##  ",
            "##      ##  ##      ##  ",
            "        ##          ##  ",
            "        ##          ##  ",
            "      ##          ##    ",
            "      ##          ##    ",
            "    ##          ##      ",
            "    ##          ##      ",
            "                        ",
            "                        ",
            "    ##          ##      ",
            "    ##          ##      ",
            "                        ",
            "                        ",
        ],
    )?;

    Ok(())
}
