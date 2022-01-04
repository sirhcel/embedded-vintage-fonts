//! Renders all characters in all sizes for debugging purposes.

use clap::Parser;
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{renderer::TextRenderer, Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_vintage_fonts::*;

/// Simple "debugger" showing the fonts contained in this crate
#[derive(Debug, Parser)]
#[clap(about, version)]
struct Args {
    /// Enable striketrough style
    #[clap(short, long)]
    strikethrough: bool,

    /// Enable underline style
    #[clap(short, long)]
    underline: bool,
}

fn main() -> Result<(), core::convert::Infallible> {
    let args = Args::parse();
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(900, 700));

    let mut character_style = MonoTextStyleBuilder::new().text_color(Rgb888::WHITE);
    if args.strikethrough {
        character_style = character_style.strikethrough_with_color(Rgb888::CSS_TOMATO);
    }
    if args.underline {
        character_style = character_style.underline_with_color(Rgb888::CSS_CORNFLOWER_BLUE);
    }
    let character_style = character_style;

    let text_style = TextStyle::default();

    #[rustfmt::skip]
    let fonts = [
        FONT_6X8,
        FONT_6X12,
        FONT_8X16,
        FONT_12X16,
        FONT_24X32,
    ];

    let mut position = Point::new(10, 10);

    for font in fonts.iter() {
        let character_style = character_style.font(font).build();

        position += Point::new(0, character_style.line_height() as i32);

        let test_text  = format!("Hello world! jpyJPY {}\n !\"#$%&'()*+,-./0123456789:;<=>?\n@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_\n`abcdefghijklmnopqrstuvwxyz{{|}}~\n\u{a0}¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿\nÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞß\nàáâãäåæçèéêëìíîïÐñòóôõö÷øùúûüýþÿ \u{ffff}", font.character_size);

        // Draw the font baseline behind the first line of text
        Line::new(
            position.y_axis(),
            position.y_axis() + display.bounding_box().size.x_axis(),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(&mut display)?;

        Text::with_text_style(&test_text, position, character_style, text_style)
            .draw(&mut display)?;

        position += font.character_size.y_axis() * test_text.lines().count() as u32;
    }

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Embedded Vintag Fonts Debugger", &output_settings).show_static(&display);

    Ok(())
}
