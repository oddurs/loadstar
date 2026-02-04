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
                        Style::default().fg(Theme::GREEN)
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

// ═══════════════════════════════════════════════════════════════════════
//  Theme — Catppuccin Mocha meets Commodore 64
//
//  Catppuccin gives us a refined palette that reads well on dark
//  terminals. We keep the green-phosphor hacker energy but use
//  it as an accent rather than painting everything in it.
// ═══════════════════════════════════════════════════════════════════════

pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // ─── Catppuccin Mocha base ─────────────────────────────────────
    pub const BASE: Color = Color::Rgb(30, 30, 46); // #1e1e2e
    pub const MANTLE: Color = Color::Rgb(24, 24, 37); // #181825
    pub const CRUST: Color = Color::Rgb(17, 17, 27); // #11111b

    pub const SURFACE0: Color = Color::Rgb(49, 50, 68); // #313244
    pub const SURFACE1: Color = Color::Rgb(69, 71, 90); // #45475a
    pub const SURFACE2: Color = Color::Rgb(88, 91, 112); // #585b70

    pub const TEXT: Color = Color::Rgb(205, 214, 244); // #cdd6f4
    pub const SUBTEXT1: Color = Color::Rgb(186, 194, 222); // #bac2de
    pub const SUBTEXT0: Color = Color::Rgb(166, 173, 200); // #a6adc8
    pub const OVERLAY2: Color = Color::Rgb(147, 153, 178); // #9399b2
    pub const OVERLAY1: Color = Color::Rgb(127, 132, 156); // #7f849c
    pub const OVERLAY0: Color = Color::Rgb(108, 112, 134); // #6c7086

    // ─── Catppuccin accent colors ──────────────────────────────────
    pub const GREEN: Color = Color::Rgb(166, 227, 161); // #a6e3a1
    pub const BLUE: Color = Color::Rgb(137, 180, 250); // #89b4fa
    pub const LAVENDER: Color = Color::Rgb(180, 190, 254); // #b4befe
    pub const MAUVE: Color = Color::Rgb(203, 166, 247); // #cba6f7
    pub const PEACH: Color = Color::Rgb(250, 179, 135); // #fab387
    pub const RED: Color = Color::Rgb(243, 139, 168); // #f38ba8
    pub const YELLOW: Color = Color::Rgb(249, 226, 175); // #f9e2af
    pub const TEAL: Color = Color::Rgb(148, 226, 213); // #94e2d5
    pub const SKY: Color = Color::Rgb(137, 220, 235); // #89dceb

    // ─── Phosphor green — the C64 accent ───────────────────────────
    pub const PHOSPHOR: Color = Color::Rgb(0, 255, 65);
    pub const PHOSPHOR_DIM: Color = Color::Rgb(0, 150, 40);
}

/// Style helpers — the old HackerTheme API, now backed by Catppuccin
pub struct HackerTheme;

#[allow(dead_code)]
impl HackerTheme {
    // Keep these aliases for any code that references them directly
    pub const FG: Color = Theme::TEXT;
    pub const FG_DIM: Color = Theme::OVERLAY0;
    pub const ACCENT: Color = Theme::BLUE;
    pub const SELECTION: Color = Color::Rgb(49, 50, 68); // SURFACE0

    // ─── Text hierarchy ────────────────────────────────────────────
    pub fn primary() -> Style {
        Style::default().fg(Theme::TEXT)
    }

    pub fn dim() -> Style {
        Style::default().fg(Theme::OVERLAY0)
    }

    pub fn muted() -> Style {
        Style::default().fg(Theme::SURFACE2)
    }

    pub fn accent() -> Style {
        Style::default().fg(Theme::BLUE)
    }

    pub fn phosphor() -> Style {
        Style::default().fg(Theme::PHOSPHOR)
    }

    // ─── Semantic ──────────────────────────────────────────────────
    pub fn error() -> Style {
        Style::default().fg(Theme::RED)
    }

    pub fn success() -> Style {
        Style::default().fg(Theme::GREEN)
    }

    pub fn warning() -> Style {
        Style::default().fg(Theme::YELLOW)
    }

    pub fn info() -> Style {
        Style::default().fg(Theme::SKY)
    }

    // ─── Borders — 3 states ────────────────────────────────────────
    pub fn border() -> Style {
        Style::default().fg(Theme::SURFACE1)
    }

    pub fn border_focused() -> Style {
        Style::default().fg(Theme::BLUE)
    }

    pub fn border_dim() -> Style {
        Style::default().fg(Theme::SURFACE0)
    }

    // ─── Selection and focus ───────────────────────────────────────
    pub fn selected() -> Style {
        Style::default()
            .bg(Theme::SURFACE0)
            .fg(Theme::TEXT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_accent() -> Style {
        Style::default()
            .bg(Theme::SURFACE0)
            .fg(Theme::GREEN)
            .add_modifier(Modifier::BOLD)
    }

    // ─── Title and chrome ──────────────────────────────────────────
    pub fn title() -> Style {
        Style::default()
            .fg(Theme::LAVENDER)
            .add_modifier(Modifier::BOLD)
    }

    pub fn brand() -> Style {
        Style::default()
            .fg(Theme::PHOSPHOR)
            .add_modifier(Modifier::BOLD)
    }

    pub fn badge() -> Style {
        Style::default()
            .fg(Theme::MAUVE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn key_hint() -> Style {
        Style::default().fg(Theme::SUBTEXT0)
    }

    pub fn key_hint_key() -> Style {
        Style::default()
            .fg(Theme::LAVENDER)
            .add_modifier(Modifier::BOLD)
    }

    // ─── Backgrounds ───────────────────────────────────────────────
    pub fn bg() -> Style {
        Style::default().bg(Theme::CRUST)
    }

    pub fn bg_surface() -> Style {
        Style::default().bg(Theme::BASE)
    }

    pub fn bg_elevated() -> Style {
        Style::default().bg(Theme::SURFACE0)
    }
}
