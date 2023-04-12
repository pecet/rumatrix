use crate::{colors::Colors, message::Message, position::*};
use rand::prelude::*;
use std::{
    cmp::max,
    io::{Stdout, Write}, cell::{RefCell, RefMut}, rc::Rc,
};
use termion::raw::RawTerminal;
use termion::{cursor, screen::AlternateScreen, style};

/// Structure to hold `FallingChar` currently to be displayed on the screen
pub struct FallingChar<'a> {
    /// Current position on the screen
    position: Position,
    /// Vector of previous positions to display trail
    previous_positions: Vec<Position>,
    /// Upper bounds for position
    max_position: Position,
    /// Chars which will be used to render both current position of `FallingChar` and its trail
    chars_to_render: Vec<char>,
    /// Colors to be used in char display
    colors: &'a Colors,
    /// Size of the trail
    size: u16,
    /// Optional message to be displayed on the screen
    message: Option<Message>,
}

impl<'a> FallingChar<'a> {
    /// Create new instance of [FallingChar]
    pub fn new(
        rng: Rc<RefCell<dyn RngCore>>,
        position: Position,
        max_position: Position,
        colors: &'a Colors,
        chars_to_use: &str,
        message: Option<Message>,
    ) -> Self {
        let size = Rc::clone(&rng).borrow_mut().gen_range(max(1, max_position.y() / 3)..=max_position.y());
        Self {
            position,
            previous_positions: Vec::with_capacity(size as usize + 1usize),
            max_position,
            chars_to_render: FallingChar::get_random_chars(rng, size, chars_to_use),
            colors,
            size,
            message,
        }
    }

    /// Get randomly ordered chars to be used in rendering process
    fn get_random_chars(rng: Rc<RefCell<dyn RngCore>>, size: u16, chars_to_use: &str) -> Vec<char> {
        // these two lines do not work when put in one line for some reason
        let rng = Rc::clone(&rng);
        let mut rng = rng.borrow_mut();
        let chars_to_select: Vec<_> = chars_to_use.chars().collect();

        let mut random_chars: Vec<char> = Vec::with_capacity(size as usize);
        while random_chars.len() < size as usize {
            let index = rng.gen_range(0..chars_to_select.len());
            let selected_char = chars_to_select[index];
            random_chars.push(selected_char);
        }
        random_chars
    }

    /// Should this instance of [FallingChar] be retained or cleaned by [FallerAdder]
    pub fn should_be_retained(&self) -> bool {
        !self
            .previous_positions
            .iter()
            .all(|&pp| pp.is_out_of_bounds(&self.max_position))
    }

    /// Render character and its trail on the `screen`
    pub fn render(&self, rng: &mut ThreadRng, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        if !self.position.is_out_of_bounds(&self.max_position) {
            let char_to_render: char = self.chars_to_render[0];
            write!(
                screen,
                "{}{}{}{}{}",
                cursor::Goto(self.position.x(), self.position.y()),
                style::Bold,
                self.colors.head.get_ansi_string(),
                char_to_render,
                style::Reset
            )
            .unwrap();
        }

        if !self.previous_positions.is_empty() {
            for (i, pos) in self.previous_positions.iter().enumerate() {
                if !pos.is_out_of_bounds(&self.max_position) {
                    let mut char_to_render = self.chars_to_render[i];
                    let mut color_to_use = self.colors.trail.get_ansi_string();
                    if i == self.size as usize - 1 {
                        color_to_use = self.colors.left_behind.get_ansi_string()
                    };
                    if i == self.previous_positions.len() - 1 {
                        char_to_render = self.chars_to_render.choose(rng).unwrap().to_owned();
                        if let Some(message) = &self.message {
                            char_to_render =
                                message.get_char_in_position(pos).unwrap_or(char_to_render);
                            if message.is_position_inside_message(pos) {
                                color_to_use = message.color.get_ansi_string();
                            }
                        }
                    }

                    write!(
                        screen,
                        "{}{}{}{}",
                        cursor::Goto(pos.x(), pos.y()),
                        color_to_use,
                        char_to_render,
                        style::Reset
                    )
                    .unwrap();
                }
            }
        }
    }

    /// Advance char position
    pub fn advance(&mut self) {
        if self.previous_positions.len() >= self.size.into() {
            let size = self.previous_positions.len();
            if size > 0 {
                self.previous_positions.remove(size - 1);
            }
        }
        self.previous_positions.insert(0, self.position);
        self.position.set_y(self.position.y() + 1);
        if let Some(ref mut message) = self.message {
            message.update_position()
        }
    }
}
