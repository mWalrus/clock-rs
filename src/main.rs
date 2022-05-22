use anyhow::Result;
use chrono::prelude::*;
use cursive::{
    reexports::enumset::enum_set,
    theme::{Color, ColorStyle, Effect, Style, Theme},
    views::{FixedLayout, TextView},
    Cursive, Rect,
};

const HEX_CODES: &'static [u8] = &[0x7E, 0x30, 0x6D, 0x79, 0x33, 0x5B, 0x5F, 0x70, 0x7F, 0x7B];

struct Digit {
    a: Rect,
    b: Rect,
    c: Rect,
    d: Rect,
    e: Rect,
    f: Rect,
    g: Rect,
    s: Vec<Style>,
}

impl Digit {
    fn new(anchor: (usize, usize), digit: u8) -> Self {
        // https://en.wikipedia.org/wiki/Seven-segment_display#/media/File:7_Segment_Display_with_Labeled_Segments.svg
        // for ordering

        // POG
        let mut styles: Vec<Style> = Vec::new();
        for i in (0..7).rev() {
            let color = if ((digit >> i) & 1) == 1 {
                Color::Rgb(255, 0, 0)
            } else {
                Color::Rgb(255, 255, 255)
            };

            let style = Style {
                effects: enum_set!(Effect::Simple),
                color: ColorStyle {
                    front: color.into(),
                    back: color.into(),
                },
            };
            styles.push(style);
        }
        Self {
            a: Rect::from_size((anchor.0 + 2, anchor.1 + 0), (10, 1)),
            b: Rect::from_size((anchor.0 + 12, anchor.1 + 1), (2, 10)),
            c: Rect::from_size((anchor.0 + 12, anchor.1 + 7), (2, 10)),
            d: Rect::from_size((anchor.0 + 2, anchor.1 + 12), (10, 1)),
            e: Rect::from_size((anchor.0 + 0, anchor.1 + 7), (2, 10)),
            f: Rect::from_size((anchor.0 + 0, anchor.1 + 1), (2, 10)),
            g: Rect::from_size((anchor.0 + 2, anchor.1 + 6), (10, 1)),
            s: styles,
        }
    }
    fn draw(&self, siv: &mut Cursive) {
        let layout = FixedLayout::new()
            .child(
                self.a,
                TextView::new(" ".repeat(self.a.width())).style(self.s[0]),
            )
            .child(
                self.b,
                TextView::new(" ".repeat(self.b.height())).style(self.s[1]),
            )
            .child(
                self.c,
                TextView::new(" ".repeat(self.c.height())).style(self.s[2]),
            )
            .child(
                self.d,
                TextView::new(" ".repeat(self.d.width())).style(self.s[3]),
            )
            .child(
                self.e,
                TextView::new(" ".repeat(self.e.height())).style(self.s[4]),
            )
            .child(
                self.f,
                TextView::new(" ".repeat(self.f.height())).style(self.s[5]),
            )
            .child(
                self.g,
                TextView::new(" ".repeat(self.g.width())).style(self.s[6]),
            );
        siv.add_fullscreen_layer(layout);
    }
}

fn main() -> Result<()> {
    let mut siv = cursive::default();
    siv.set_theme(Theme {
        shadow: false,
        ..Default::default()
    });
    siv.add_global_callback('q', |s| s.quit());
    siv.set_fps(10);
    ui(&mut siv);
    siv.run();
    Ok(())
}

fn ui(siv: &mut Cursive) {
    let digits = take_digits();
    let (h1, h2, m1, m2) = (
        HEX_CODES[digits.0],
        HEX_CODES[digits.1],
        HEX_CODES[digits.2],
        HEX_CODES[digits.3],
    );

    // NOTE: draw in reverse order to not render them over each other
    Digit::new((48, 0), m2).draw(siv);
    Digit::new((32, 0), m1).draw(siv);
    Digit::new((16, 0), h2).draw(siv);
    Digit::new((0, 0), h1).draw(siv);
}

fn take_digits() -> (usize, usize, usize, usize) {
    let time = Local::now().time().format("%H:%M").to_string();
    let (h, m) = time.split_once(':').unwrap();
    let mut h = h.chars();
    let mut m = m.chars();
    let (h1, h2) = (
        h.next().unwrap().to_digit(10).unwrap(),
        h.next().unwrap().to_digit(10).unwrap(),
    );
    let (m1, m2) = (
        m.next().unwrap().to_digit(10).unwrap(),
        m.next().unwrap().to_digit(10).unwrap(),
    );
    (h1 as usize, h2 as usize, m1 as usize, m2 as usize)
}
