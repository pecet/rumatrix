use serde::{Deserialize, Serialize};

use crate::message::TextType;

/// Basic structure to hold position on the screen
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    /// x coordinate
    pub x: u16,
    /// y coordinate
    pub y: u16,
}

impl Position {
    /// Check if `self` position is outside of position defined by `bounds`
    #[inline]
    pub fn is_out_of_bounds(&self, bounds: &Position) -> bool {
        self.y > bounds.y || self.x > bounds.x
    }

    /// Returns centered position on screen or [None] if cannot center text
    #[inline]
    pub fn new_for_centered_text(bounds: &Position, text: &TextType) -> Option<Self> {
        if bounds.x < text.to_string().len() as u16 {
            return None;
        }
        Some(Self {
            x: (bounds.x - text.to_string().len() as u16) / 2,
            y: bounds.y / 2,
        })
    }
}
