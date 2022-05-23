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
    let mut colon_on = true;
    let mut fps_counter: u32 = 0;
    loop {
        let clock_layout = Clock::new(colon_on).layout();
        siv.add_layer(clock_layout);
        if !siv.is_running() {
            break;
        }
        siv.step();
        siv.pop_layer();

        if fps_counter == FPS {
            colon_on = !colon_on;
            fps_counter = 0;
        } else {
            fps_counter += 1;
        }
    }
    Ok(())
}
