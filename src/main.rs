mod clock;

use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use clock::Clock;
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{Palette, Theme};
use cursive::CursiveRunnable;

const FPS: u32 = 27;

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

    let tick_rate = Duration::from_millis(1000);

    let mut clock = Clock::new();

    let mut last_tick = Instant::now();
    loop {
        siv.add_layer(clock.layout());
        if !siv.is_running() {
            break;
        }
        siv.step();
        siv.pop_layer();

        if last_tick.elapsed() >= tick_rate {
            clock.toggle_colon();
            last_tick = Instant::now();
        }

        clock.update();
    }
    Ok(())
}
