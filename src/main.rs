use anyhow::Result;
use chrono::prelude::*;
use cursive::direction::Orientation;
use cursive::event::Event;
use cursive::theme::BaseColor::*;
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;
use cursive::theme::*;
use cursive::views::LinearLayout;
use cursive::CursiveRunnable;
use cursive::Printer;
use cursive::View;
use cursive::{
    reexports::enumset::enum_set,
    theme::{Color, ColorStyle, Effect, Palette, Style, Theme},
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

struct Colon {
    a: Rect,
    b: Rect,
    s: Style,
}

struct Clock {
    d1: Digit,
    d2: Digit,
    cl: Colon,
    d3: Digit,
    d4: Digit,
}

impl Clock {
    fn new() -> Self {
        let digits = take_digits();
        let (h1, h2, m1, m2) = (
            HEX_CODES[digits.0],
            HEX_CODES[digits.1],
            HEX_CODES[digits.2],
            HEX_CODES[digits.3],
        );

        Self {
            d1: Digit::new((2, 0), m2),
            d2: Digit::new((2, 0), m1),
            cl: Colon::new((1, 0)),
            d3: Digit::new((2, 0), h2),
            d4: Digit::new((0, 0), h1),
        }
    }

    fn layout(&self) -> LinearLayout {
        LinearLayout::new(Orientation::Horizontal)
            .child(self.d4.layout())
            .child(self.d3.layout())
            .child(self.cl.layout())
            .child(self.d2.layout())
            .child(self.d1.layout())
    }
}

impl Colon {
    fn new(anchor: (usize, usize)) -> Self {
        let style = Style {
            effects: enum_set!(Effect::Simple),
            color: ColorStyle {
                front: Color::Dark(Green).into(),
                back: Color::Dark(Green).into(),
            },
        };
        Self {
            a: Rect::from_size((anchor.0 + 1, anchor.1 + 3), (2, 2)),
            b: Rect::from_size((anchor.0 + 1, anchor.1 + 9), (2, 2)),
            s: style,
        }
    }

    fn layout(&self) -> FixedLayout {
        FixedLayout::new()
            .child(self.a, TextView::new("  ").style(self.s))
            .child(self.b, TextView::new("  ").style(self.s))
    }
}

impl Digit {
    fn new(anchor: (usize, usize), digit: u8) -> Self {
        // https://en.wikipedia.org/wiki/Seven-segment_display#/media/File:7_Segment_Display_with_Labeled_Segments.svg
        // for ordering

        // POG
        let mut styles: Vec<Style> = Vec::new();
        for i in (0..7).rev() {
            let color = if ((digit >> i) & 1) == 1 {
                Color::Dark(Green)
            } else {
                Color::Dark(Black)
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
    fn layout(&self) -> FixedLayout {
        FixedLayout::new()
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
            )
    }
}

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
    siv.set_fps(30);
    // in the loop we want to:
    // 1. clear screen
    // 2. Define a new clock layout with the current time
    // 3. Draw that clock layout
    loop {
        let clock_layout = Clock::new().layout();
        siv.add_layer(clock_layout);
        if !siv.is_running() {
            break;
        }
        siv.step();
        siv.pop_layer();
    }
    Ok(())
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
