use crate::Position;

#[derive(Clone)]
pub struct Message {
    pub position: Position,
    pub text: String
}

impl Message {
    fn is_position_inside_message(&self, other_position: &Position) -> bool {
        other_position.y == self.position.y &&
        other_position.x >= self.position.x &&
        other_position.x < self.position.x + self.text.len() as u16
    }

    fn get_message_char(&self, other_position: &Position) -> char {
        let nth = (other_position.x - self.position.x) as usize;
        self.text.chars().nth(nth).unwrap()
    }

    pub fn get_char_in_position(&self, other_position: &Position) -> Option<char> {
        if self.is_position_inside_message(other_position) {
            Some(self.get_message_char(other_position))
        } else {
            None
        }
    }
}