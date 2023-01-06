use derive_getters::Getters;
use serde::{Serialize, Deserialize};
use termion::color;

/// Colors used for displaying [FallingChar]
#[derive(Clone, Serialize, Deserialize)]
pub struct Colors {
    /// Trail [Color]
    pub trail: Color,
    /// Head (first char) [Color]
    pub head: Color,
}

/// Enum for Color
#[derive(Clone, Serialize, Deserialize)]
pub enum Color {
    /// Color from 16-color palette of terminal
    Palette(u8),
    /// RGB color
    RGB {
        /// Red
        r: u8,
        /// Green
        g: u8,
        /// Blue
        b: u8,
    },
}

impl Color {
    /// Create [Color::RGB] from vector `rgb` values, only first three values will be used (indices 0 to 2)
    ///
    /// This does NOT check if vector has at least 3 values, so might result in panic while accessing elements
    pub fn rgb_from_vec(rgb: Vec<u8>) -> Color {
        Color::RGB { r: rgb[0], g: rgb[1], b: rgb[2] }
    }

    /// Get ANSI string for [Color]
    pub fn get_ansi_string(&self) -> String {
        match self {
            Color::Palette(color) => {
                match color {
                    1 => color::Red.fg_str(),
                    2 => color::Green.fg_str(),
                    3 => color::Yellow.fg_str(),
                    4 => color::Blue.fg_str(),
                    5 => color::Magenta.fg_str(),
                    6 => color::Cyan.fg_str(),
                    7 => color::White.fg_str(),
                    8 => color::LightBlack.fg_str(),
                    9 => color::LightRed.fg_str(),
                    10 => color::LightGreen.fg_str(),
                    11 => color::LightYellow.fg_str(),
                    12 => color::LightBlue.fg_str(),
                    13 => color::LightMagenta.fg_str(),
                    14 => color::LightCyan.fg_str(),
                    15 => color::LightWhite.fg_str(),
                    _ => color::Black.fg_str(),
                }.to_owned()
            }
            Color::RGB { r, g, b } => {
                color::Rgb(*r, *g, *b).fg_string()
            }
        }
    }

    /// Get default alternate color based on `self` color
    pub fn get_alternate_color(&self) -> Color {
        match self {
            Color::Palette(color) => {
                match color {
                    0..=7 => Color::Palette(color + 8),
                    8..=15 => Color::Palette(color - 8),
                    _ => self.clone(), // this should never happen, but just in case...
                }
            }
            Color::RGB { r, g, b } => {
                let mut r = *r as u16;
                let mut g = *g as u16;
                let mut b = *b as u16;
                let hardcoded_offset = 15;
                r += hardcoded_offset;
                g += hardcoded_offset;
                b += hardcoded_offset;
                if r > 255 {
                    r = 255;
                }
                if g > 255 {
                    g = 255;
                }
                if b > 255 {
                    b = 255;
                }
                let r = r as u8;
                let g = g as u8;
                let b = b as u8;
                Color::RGB {r, g, b}
            }
        }
    }
}