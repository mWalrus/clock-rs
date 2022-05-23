mod clock;

use anyhow::Result;
use clock::Clock;
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::{Palette, Theme};
use cursive::CursiveRunnable;

const FPS: u32 = 30;

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
    // step the event loop manually
    let mut fps_counter: u32 = 0;
    let mut clock = Clock::new();
    loop {
        siv.add_layer(clock.layout());
        if !siv.is_running() {
            break;
        }
        siv.step();
        siv.pop_layer();

        if fps_counter == FPS {
            clock.toggle_colon();
            fps_counter = 0;
        } else {
            fps_counter += 1;
        }
        clock.update();
    }
    Ok(())
}
