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
    #[serde(default)]
    #[serde(skip_serializing)]
    last_bounds: Position,    
}

impl CenteredPosition {
    /// New [CenteredPosition]
    pub fn new(bounds: &Position, text: &TextType) -> Self {
        let mut new_centered = Self {
            position: Position::default(),
            last_text: TextType::StaticString("".into()),
            last_bounds: Position::default(),
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
        if *text != self.last_text || self.last_bounds != *bounds {
            if bounds.x < text.to_string().len() as u16 {
                // TO DO: change return type to result and return error here
                panic!("Message provided is longer than screen");
            }
            let x = (bounds.x - text.to_string().len() as u16) / 2;
            let y = bounds.y / 2;
            self.position.x = x;
            self.position.y = y;
            self.last_text = text.clone();
            self.last_bounds = *bounds;
        }
    }
}

impl Default for CenteredPosition {
    fn default() -> Self {
        Self {
            position: Default::default(),
            last_text: TextType::StaticString("".into()),
            last_bounds: Position::default(),
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
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_new_and_getters() {
        let position = Position::new(12, 24);
        assert_eq!(position.x(), 12);
        assert_eq!(position.y(), 24);
    }

    #[test]
    fn position_setters_and_getters() {
        let mut position = Position::new(0, 0);
        position.set_x(3);
        position.set_y(15);
        assert_eq!(position.x(), 3);
        assert_eq!(position.y(), 15);        
    }

    #[test]
    fn position_is_out_of_bounds() {
        let bounds = Position::new(30, 30);
        let position = Position::new(15, 15);
        assert_eq!(position.is_out_of_bounds(&bounds), false);
        let position = Position::new(333, 2);
        assert_eq!(position.is_out_of_bounds(&bounds), true);
        let position = Position::new(2, 333);
        assert_eq!(position.is_out_of_bounds(&bounds), true);
    }

    #[test]
    fn centered_position_with_static_text() {
        let bounds = Position::new(30, 30);
        let text = TextType::StaticString("X".to_owned());
        let position = CenteredPosition::new(&bounds, &text);
        assert_eq!(position.x(), 14);
        assert_eq!(position.y(), 15);
    }

    #[test]
    fn centered_position_with_static_text_updating() {
        let bounds = Position::new(30, 30);
        let text = TextType::StaticString("X".to_owned());
        let mut position = CenteredPosition::new(&bounds, &text);
        let bounds = Position::new(31, 22);
        let text = TextType::StaticString("Lorem Ipsum".to_owned());
        position.update(&bounds, &text);
        assert_eq!(position.x(), 10);
        assert_eq!(position.y(), 11);
    }
}