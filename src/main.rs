mod clock;

use anyhow::Result;
use clock::Clock;
use cursive::theme::BaseColor::*;
use cursive::theme::Color::{self, *};
use cursive::theme::PaletteColor::*;
use cursive::theme::{Palette, Theme};
use cursive::CursiveRunnable;
use std::time::Duration;
use std::time::Instant;

// change these to whatever you like
const ON_COLOR: Color = Color::Dark(Magenta);
const OFF_COLOR: Color = Color::Dark(Black);

const FPS: u32 = 30;
const HEX_CODES: &'static [u8] = &[0x7E, 0x30, 0x6D, 0x79, 0x33, 0x5B, 0x5F, 0x70, 0x7F, 0x7B];
const SEGMENTS: usize = 7;

fn main() -> Result<()> {
    let mut palette = Palette::default();
    palette[Background] = TerminalDefault;
    palette[View] = TerminalDefault;

    let siv = cursive::default();
    let mut siv = CursiveRunnable::into_runner(siv);
    siv.set_theme(Theme {
        shadow: false,
        palette,
        ..Default::default()
    });
    siv.add_global_callback('q', |s| s.quit());

    siv.set_fps(FPS);

    let mut clock = Clock::new();

    let colon_toggle_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    loop {
        siv.add_layer(clock.layout());
        if !siv.is_running() {
            break;
        }
        siv.step();
        siv.pop_layer();

        if last_tick.elapsed() >= colon_toggle_rate {
            clock.toggle_colon();
            last_tick = Instant::now();
        }

        clock.update();
    }
    Ok(())
}
