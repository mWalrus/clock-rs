use chrono::prelude::*;
use cursive::direction::Orientation;
use cursive::theme::BaseColor::*;
use cursive::views::LinearLayout;
use cursive::{
    reexports::enumset::enum_set,
    theme::{Color, ColorStyle, Effect, Style},
    views::{FixedLayout, TextView},
    Rect,
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
    on: bool,
}

pub struct Clock {
    d1: Digit,
    d2: Digit,
    cl: Colon,
    d3: Digit,
    d4: Digit,
}

impl Clock {
    pub fn new() -> Self {
        let (h1, h2, m1, m2) = take_digits();
        Self {
            d1: Digit::new((2, 0), m2),
            d2: Digit::new((2, 0), m1),
            cl: Colon::new((1, 0)),
            d3: Digit::new((2, 0), h2),
            d4: Digit::new((0, 0), h1),
        }
    }

    pub fn update(&mut self) {
        let (h1, h2, m1, m2) = take_digits();
        self.d1 = Digit::new((2, 0), m2);
        self.d2 = Digit::new((2, 0), m1);
        self.d3 = Digit::new((2, 0), h2);
        self.d4 = Digit::new((0, 0), h1);
    }

    pub fn toggle_colon(&mut self) {
        self.cl.toggle();
    }

    pub fn layout(&self) -> LinearLayout {
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
                front: Color::TerminalDefault.into(),
                back: Color::Dark(Green).into(), // on color
            },
        };
        Self {
            a: Rect::from_size((anchor.0 + 1, anchor.1 + 3), (2, 2)),
            b: Rect::from_size((anchor.0 + 1, anchor.1 + 9), (2, 2)),
            s: style,
            on: false,
        }
    }

    fn toggle(&mut self) {
        self.s.color.back = if self.on {
            Color::Dark(Green).into() // on color
        } else {
            Color::Dark(Black).into()
        };
        self.on = !self.on
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

fn take_digits() -> (u8, u8, u8, u8) {
    let time = Local::now().time().format("%H:%M").to_string();
    let (h, m) = time.split_once(':').unwrap();
    let mut h = h.chars();
    let mut m = m.chars();
    let (h1, h2) = (
        h.next().unwrap().to_digit(10).unwrap() as usize,
        h.next().unwrap().to_digit(10).unwrap() as usize,
    );
    let (m1, m2) = (
        m.next().unwrap().to_digit(10).unwrap() as usize,
        m.next().unwrap().to_digit(10).unwrap() as usize,
    );
    (HEX_CODES[h1], HEX_CODES[h2], HEX_CODES[m1], HEX_CODES[m2])
}
