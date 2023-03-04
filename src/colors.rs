use serde::{Deserialize, Serialize};
use termion::color;

/// Colors used for displaying [FallingChar]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Colors {
    /// Trail [Color]
    pub trail: Color,
    /// Head (first char) [Color]
    pub head: Color,
    /// [Color] of characters left behind
    pub left_behind: Color,
}

/// Enum for Color
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
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

macro_rules! add_offset_to_u8 {
    ($num: ident, $offset: ident) => {
        let mut $num = *$num as i16;
        $num += $offset;
        if $num > 255 {
            $num = 255;
        } else if $num < 0 {
            $num = 0;
        }
        let $num = $num as u8;
    };
}

impl Color {
    /// Create [Color::RGB] from vector `rgb` values, only first three values will be used (indices 0 to 2)
    ///
    /// This does NOT check if vector has at least 3 values, so might result in panic while accessing elements
    pub fn rgb_from_vec(rgb: Vec<u8>) -> Color {
        Color::RGB {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }

    /// Get ANSI string for [Color]
    pub fn get_ansi_string(&self) -> String {
        match self {
            Color::Palette(color) => match color {
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
            }
            .to_owned(),
            Color::RGB { r, g, b } => color::Rgb(*r, *g, *b).fg_string(),
        }
    }

    /// Get default head color based on `self` color
    pub fn get_auto_head_color(&self) -> Color {
        match self {
            Color::Palette(color) => {
                match color {
                    0..=7 => Color::Palette(color + 8),
                    8..=15 => Color::Palette(color - 8),
                    _ => self.clone(), // this should never happen, but just in case...
                }
            }
            Color::RGB { r, g, b } => {
                let color_offset = 15i16;

                add_offset_to_u8!(r, color_offset);
                add_offset_to_u8!(g, color_offset);
                add_offset_to_u8!(b, color_offset);
                Color::RGB { r, g, b }
            }
        }
    }

    /// Get default left behind color based on `self` color
    pub fn get_auto_left_behind_color(&self) -> Color {
        match self {
            Color::Palette(color) => Color::Palette(*color),
            Color::RGB { r, g, b } => {
                let color_offset = -30i16;

                add_offset_to_u8!(r, color_offset);
                add_offset_to_u8!(g, color_offset);
                add_offset_to_u8!(b, color_offset);
                Color::RGB { r, g, b }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rgb_ansi_string() {
        let c = Color::RGB { r: 10, g: 20, b: 30 };
        let ansi_string = c.get_ansi_string();
        assert_eq!(ansi_string, "\u{1b}[38;2;10;20;30m");
    }

    #[test]
    fn rgb_from_vec() {
        let c = Color::RGB { r: 2, g: 4, b: 8 };
        match c {
            Color::RGB { r, g, b } => {
                assert_eq!(r, 2);
                assert_eq!(g, 4);
                assert_eq!(b, 8);
            },
            _ => { panic!("Got non RGB color"); }
        }
    }

    #[test]
    fn rgb_auto_head_color() {
        let c = Color::RGB { r: 255, g: 250, b: 10 };
        let head_color = c.get_auto_head_color();
        match head_color {
            Color::RGB { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 255);
                assert_eq!(b, 25);
            },
            _ => { panic!("Got non RGB color"); }
        }
    }

    #[test]
    fn rgb_left_behind_color() {
        let c = Color::RGB { r: 255, g: 250, b: 10 };
        let left_behind_color = c.get_auto_left_behind_color();
        match left_behind_color {
            Color::RGB { r, g, b } => {
                assert_eq!(r, 225);
                assert_eq!(g, 220);
                assert_eq!(b, 0);
            },
            _ => { panic!("Got non RGB color"); }
        }
    }

    #[test]
    fn pallete_ansi_string() {
        let c = Color::Palette(3);
        let ansi_string = c.get_ansi_string();
        assert_eq!(ansi_string, "\u{1b}[38;5;3m")
    }

    #[test]
    fn pallete_head_color() {
        let c = Color::Palette(4);
        let head_color = c.get_auto_head_color();
        match head_color {
            Color::Palette(p) => {
                assert_eq!(p, 12);
            },
            _ => { panic!("Got non Palette color"); }
        }
    }

    #[test]
    fn pallete_auto_left_behind_color() {
        let c = Color::Palette(4);
        let left_behind_color = c.get_auto_head_color();
        match left_behind_color {
            Color::Palette(p) => {
                assert_eq!(p, 12);
            },
            _ => { panic!("Got non Palette color"); }
        }
    }
}
