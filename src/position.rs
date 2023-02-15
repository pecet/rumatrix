use crate::message::TextType;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

/// Trait definies basic methods for all position types. See also [PositionType]
#[enum_dispatch]
pub trait PositionTrait {
    /// Get x value
    fn x(&self) -> u16;
    /// Get y value
    fn y(&self) -> u16;
    /// Set x value
    fn set_x(&mut self, x: u16);
    /// Set y value
    fn set_y(&mut self, y: u16);
    /// Is position outside of bounds defined by another position?
    fn is_out_of_bounds(&self, bounds: &Position) -> bool {
        self.y() > bounds.y() || self.x() > bounds.x()
    }
    /// Update the position based on text
    fn update(&mut self, _bounds: &Position, _text: &TextType) {
        // do nothing by default
    }
}

/// Basic structure to hold position on the screen
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
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

impl Position {
    /// Create new [Position]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Position which autocenters itself
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
    /// New [CenteredPosition]
    pub fn new(bounds: &Position, text: &TextType) -> Self {
        let mut new_centered = Self {
            position: Position { x: 0, y: 0 },
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
        /*if *text != self.last_text*/ {
            if bounds.x < text.to_string().len() as u16 {
                // TO DO: change return type to result and return error here
                panic!("Message provided is longer than screen");
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
            last_text: TextType::StaticString("".into()),
        }
    }
}

/// Position type
#[enum_dispatch(PositionTrait)]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionType {
    /// Static [Position]
    Static(Position),
    /// [CenteredPostion]
    Center(CenteredPosition),
}
