use crate::{HEX_CODES, OFF_COLOR, ON_COLOR, SEGMENTS};
use chrono::prelude::*;
use cursive::direction::Orientation;
use cursive::views::LinearLayout;
use cursive::{
    reexports::enumset::enum_set,
    theme::{Color, ColorStyle, Effect, Style},
    views::{FixedLayout, TextView},
    Rect,
};

type IsActive = bool;

struct Digit {
    a: Rect,
    b: Rect,
    c: Rect,
    d: Rect,
    e: Rect,
    f: Rect,
    g: Rect,
    segment_states: Vec<IsActive>,
    style: Style,
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
            d4: Digit::new((2, 0), h1),
        }
    }

    pub fn update(&mut self) {
        let (h1, h2, m1, m2) = take_digits();
        self.d1.update(m2);
        self.d2.update(m1);
        self.d4.update(h2);
        self.d3.update(h1);
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
                back: ON_COLOR.into(),
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
            ON_COLOR.into()
        } else {
            OFF_COLOR.into()
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

        let mut dgt = Self {
            a: Rect::from_size((anchor.0, anchor.1 + 0), (14, 1)),
            b: Rect::from_size((anchor.0 + 12, anchor.1), (2, 14)),
            c: Rect::from_size((anchor.0 + 12, anchor.1 + 6), (2, 14)),
            d: Rect::from_size((anchor.0, anchor.1 + 12), (14, 1)),
            e: Rect::from_size((anchor.0 + 0, anchor.1 + 6), (2, 14)),
            f: Rect::from_size((anchor.0 + 0, anchor.1), (2, 14)),
            g: Rect::from_size((anchor.0, anchor.1 + 6), (14, 1)),
            segment_states: Vec::with_capacity(SEGMENTS),
            style: Style {
                effects: enum_set!(Effect::Simple),
                color: ColorStyle {
                    front: ON_COLOR.into(),
                    back: ON_COLOR.into(),
                },
            },
        };
        dgt.update(digit);
        dgt
    }

    fn update(&mut self, digit: u8) {
        self.segment_states.clear();

        for i in (0..7).rev() {
            let is_active = ((digit >> i) & 1) == 1;

            self.segment_states.push(is_active);
        }
    }

    fn layout(&self) -> FixedLayout {
        let mut layout = FixedLayout::new();
        if self.segment_states[0] {
            layout = layout.child(
                self.a,
                TextView::new(" ".repeat(self.a.width())).style(self.style),
            );
        }
        if self.segment_states[1] {
            layout = layout.child(
                self.b,
                TextView::new(" ".repeat(self.b.height())).style(self.style),
            );
        }
        if self.segment_states[2] {
            layout = layout.child(
                self.c,
                TextView::new(" ".repeat(self.c.height())).style(self.style),
            );
        }
        if self.segment_states[3] {
            layout = layout.child(
                self.d,
                TextView::new(" ".repeat(self.d.width())).style(self.style),
            );
        }
        if self.segment_states[4] {
            layout = layout.child(
                self.e,
                TextView::new(" ".repeat(self.e.height())).style(self.style),
            );
        }
        if self.segment_states[5] {
            layout = layout.child(
                self.f,
                TextView::new(" ".repeat(self.f.height())).style(self.style),
            );
        }
        if self.segment_states[6] {
            layout = layout.child(
                self.g,
                TextView::new(" ".repeat(self.g.width())).style(self.style),
            );
        }
        layout
    }
}

// TODO: move this into the Clock struct
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
