//! Visual effects for the hacker TUI
//! Glitch effects, matrix rain, and other visual candy

use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use crate::ascii_art::MATRIX_CHARS;

/// Matrix rain effect
pub struct MatrixRain {
    columns: Vec<MatrixColumn>,
    height: u16,
}

struct MatrixColumn {
    chars: Vec<char>,
    head: i32,
    speed: u8,
    length: u8,
}

impl MatrixRain {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        let columns: Vec<MatrixColumn> = (0..width)
            .map(|_| MatrixColumn {
                chars: (0..height)
                    .map(|_| MATRIX_CHARS[rng.gen_range(0..MATRIX_CHARS.len())])
                    .collect(),
                head: rng.gen_range(-(height as i32)..0),
                speed: rng.gen_range(1..4),
                length: rng.gen_range(5..15),
            })
            .collect();

        Self { columns, height }
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        for col in &mut self.columns {
            col.head += col.speed as i32;

            if col.head > (self.height as i32 + col.length as i32) {
                col.head = rng.gen_range(-(self.height as i32)..-(col.length as i32));
                col.speed = rng.gen_range(1..4);
                col.length = rng.gen_range(5..15);
            }

            if rng.gen_bool(0.1) {
                let idx = rng.gen_range(0..col.chars.len());
                col.chars[idx] = MATRIX_CHARS[rng.gen_range(0..MATRIX_CHARS.len())];
            }
        }
    }
}

impl Widget for &MatrixRain {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (x, col) in self.columns.iter().enumerate() {
            if x >= area.width as usize {
                break;
            }

            for y in 0..area.height {
                let dist_from_head = col.head - y as i32;

                if dist_from_head >= 0 && dist_from_head < col.length as i32 {
                    let char_idx = (y as usize) % col.chars.len();
                    let ch = col.chars[char_idx];

                    let style = if dist_from_head == 0 {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else if dist_from_head < 3 {
                        Style::default().fg(Color::LightGreen)
                    } else {
                        let brightness = 255 - ((dist_from_head as u8).saturating_mul(20));
                        Style::default().fg(Color::Rgb(0, brightness, 0))
                    };

                    buf.get_mut(area.x + x as u16, area.y + y)
                        .set_char(ch)
                        .set_style(style);
                }
            }
        }
    }
}

/// Typing effect state
pub struct TypeWriter {
    pub full_text: String,
    pub visible_chars: usize,
    pub cursor_visible: bool,
    pub complete: bool,
}

impl TypeWriter {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            full_text: text.into(),
            visible_chars: 0,
            cursor_visible: true,
            complete: false,
        }
    }

    pub fn tick(&mut self) {
        if self.visible_chars < self.full_text.len() {
            self.visible_chars += 1;
        } else {
            self.complete = true;
        }
        self.cursor_visible = !self.cursor_visible;
    }

    pub fn visible_text(&self) -> &str {
        let end = self.visible_chars.min(self.full_text.len());
        &self.full_text[..end]
    }

    pub fn cursor(&self) -> &str {
        if self.cursor_visible && !self.complete {
            "█"
        } else if self.cursor_visible {
            "_"
        } else {
            " "
        }
    }

    pub fn skip(&mut self) {
        self.visible_chars = self.full_text.len();
        self.complete = true;
    }
}

/// Spinner animation
pub struct Spinner {
    frames: &'static [&'static str],
    current: usize,
}

impl Spinner {
    pub fn braille() -> Self {
        Self {
            frames: crate::ascii_art::SPINNER_BRAILLE,
            current: 0,
        }
    }

    pub fn tick(&mut self) {
        self.current = (self.current + 1) % self.frames.len();
    }

    pub fn current(&self) -> &'static str {
        self.frames[self.current]
    }
}

/// Color scheme for the hacker theme
pub struct HackerTheme;

impl HackerTheme {
    pub const FG: Color = Color::Rgb(0, 255, 65);
    pub const FG_DIM: Color = Color::Rgb(0, 150, 40);
    pub const ACCENT: Color = Color::Cyan;
    pub const SELECTION: Color = Color::Rgb(30, 60, 40);

    pub fn primary() -> Style {
        Style::default().fg(Self::FG)
    }

    pub fn dim() -> Style {
        Style::default().fg(Self::FG_DIM)
    }

    pub fn accent() -> Style {
        Style::default().fg(Self::ACCENT)
    }

    pub fn selected() -> Style {
        Style::default().bg(Self::SELECTION).fg(Self::FG)
    }

    pub fn error() -> Style {
        Style::default().fg(Color::Red)
    }

    pub fn success() -> Style {
        Style::default().fg(Color::LightGreen)
    }

    pub fn warning() -> Style {
        Style::default().fg(Color::Yellow)
    }

    pub fn title() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border() -> Style {
        Style::default().fg(Self::FG_DIM)
    }
}
