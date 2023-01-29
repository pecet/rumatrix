use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::message::TextType;

#[enum_dispatch]
pub trait PositionTrait {
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    fn set_x(&mut self, x: u16);
    fn set_y(&mut self, y: u16);
    fn is_out_of_bounds(&self, bounds: &Position) -> bool {
        self.y() > bounds.y() || self.x() > bounds.x()
    }
    fn update(&mut self, bounds: &Position, text: &TextType) {
        // do nothing by default
    }
}

/// Basic structure to hold position on the screen
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    /// x coordinate
    x: u16,
    /// y coordinate
    y: u16,
}

impl PositionTrait for Position {
    fn x(&self) -> u16 {
        self.x
    }
    fn y(&self) -> u16 {
        self.y
    }
    fn set_x(&mut self, x: u16) {
        self.x = x;
    }
    fn set_y(&mut self, y: u16) {
        self.y = y;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CenteredPosition {
    #[serde(default)]
    #[serde(skip_serializing)]
    position: Position,
    #[serde(default)]
    #[serde(skip_serializing)]
    last_text: TextType,
}

impl CenteredPosition {
    pub fn new(bounds: &Position, text: &TextType) -> Self {
        let mut new_centered = Self {
            position: Position {x: 0, y: 0},
            last_text: TextType::StaticString("".into()),
        };
        new_centered.update(bounds, text);
        new_centered
    }
}

impl PositionTrait for CenteredPosition {
    fn x(&self) -> u16 {
        self.position.x()
    }

    fn y(&self) -> u16 {
        self.position.y()
    }

    fn set_x(&mut self, x: u16) {
        self.position.set_x(x);
    }

    fn set_y(&mut self, y: u16) {
        self.position.set_y(y);
    }

    fn update(&mut self, bounds: &Position, text: &TextType) {
        // Update only if necessary
        if *text != self.last_text {
            if bounds.x < text.to_string().len() as u16 {
                // TO DO: change return type to result and return error here
                return
            }
            let x = (bounds.x - text.to_string().len() as u16) / 2;
            let y = bounds.y / 2;
            self.position.x = x;
            self.position.y = y;
        }
    }
}

impl Default for CenteredPosition {
    fn default() -> Self {
        Self {
            position: Default::default(),
            last_text: TextType::StaticString("".into())
        }
    }
}

#[enum_dispatch(PositionTrait)]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionType {
    Static(Position),
    Center(CenteredPosition),
}
