#[derive(Clone, Copy, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn is_out_of_bounds(&self, bounds: &Position) -> bool {
        self.y > bounds.y || self.x > bounds.x
    }

}