use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{colors::Color, Position};
use crate::message::TextType::StaticString;
use crate::position::{CenteredPosition, PositionTrait, PositionType};

/// Struct holds message currently displayed on screen with its:
/// `position` and `text`
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// [PositionType] of message on the screen
    pub position: PositionType,
    /// Bounds max [Position]
    pub bounds: Position,
    /// Text of the message
    pub text: TextType,
    /// [Color] of message
    pub color: Color,
}

impl Message {
    /// Returns centered message wrapped in [Some] or [None] if not possible to center
    pub fn new_centered_or_none(bounds: Position, text: TextType, color: Color) -> Option<Self> {
        let position = PositionType::Center(CenteredPosition::new(&bounds, &text));
        Some(Message {
            position,
            text,
            color,
            bounds,
        })
    }

    /// Check if `other_position` is inside of message's `position`
    pub fn is_position_inside_message(&self, other_position: &Position) -> bool {
        other_position.y() == self.position.y()
            && other_position.x() >= self.position.x()
            && other_position.x() < self.position.x() + self.text.to_string().len() as u16
    }

    /// Get [char], use it only if `is_position_inside_message` is true.
    /// Otherwise it might panic when calculating [char] to get
    fn get_message_char(&self, other_position: &Position) -> char {
        let nth = (other_position.x() - self.position.x()) as usize;
        self.text.to_string().chars().nth(nth).unwrap()
    }

    /// Check if `other_position` is inside of message's `position`
    ///
    /// If true: return [Some] with char to be displayed
    ///
    /// If false: return [None]
    pub fn get_char_in_position(&self, other_position: &Position) -> Option<char> {
        if self.is_position_inside_message(other_position) {
            Some(self.get_message_char(other_position))
        } else {
            None
        }
    }

    pub fn update_position(&mut self) {
        self.position.update(&self.bounds, &self.text)
    }
}

/// [TextType] of text to display on the screen.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextType {
    // Basic static string
    StaticString(String),
    // Current Date and/or Time with formatting string
    CurrentDateTime(String),
}

impl Default for TextType {
    fn default() -> Self {
        StaticString("".into())
    }
}

impl ToString for TextType {
    fn to_string(&self) -> String {
        match self {
            TextType::StaticString(ref text) => text.clone(),
            TextType::CurrentDateTime(ref format) => {
                let date = Local::now();
                let format = date.format(format);
                format!("{format}")
            }
        }
    }
}


