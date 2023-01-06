use serde::{Serialize, Deserialize};

/// Basic structure to hold position on the screen
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    /// x coordinate
    pub x: u16,
    /// y coordinate
    pub y: u16,
}

impl Position {
    /// Check if `self` position is outside of position defined by `bounds`
    pub fn is_out_of_bounds(&self, bounds: &Position) -> bool {
        self.y > bounds.y || self.x > bounds.x
    }
}