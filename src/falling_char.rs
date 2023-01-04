use crate::{position::*, message::Message};
use rand::prelude::*;
use std::{
    cmp::max,
    io::{Stdout, Write},
};
use termion::raw::RawTerminal;
use termion::{cursor, screen::AlternateScreen, style};

/// Structure to hold `FallingChar` currently to be displayed on the screen
pub struct FallingChar {
    /// Current position on the screen
    position: Position,
    /// Vector of previous positions to display trail
    previous_positions: Vec<Position>,
    /// Upper bounds for position
    max_position: Position,
    /// Chars which will be used to render both current position of `FallingChar` and its trail
    chars_to_render: Vec<char>,
    /// ANSI formatting string for displaying trail's color
    color_fmt: String,
    /// ANSI formatting string for displaying current position's color
    color_lighter_fmt: String,
    /// Size of the trail
    size: u16,
    /// Optional message to be displayed on the screen
    message: Option<Message>,
}

impl FallingChar {
    /// Create new instance of [FallingChar]
    pub fn new(
        rng: &mut ThreadRng,
        position: Position,
        max_position: Position,
        color_fmt: String,
        color_lighter_fmt: String,
        chars_to_use: &str,
        message: Option<Message>,
    ) -> Self {
        let size = rng.gen_range(max(1, max_position.y / 3)..=max_position.y);
        Self {
            position,
            previous_positions: Vec::with_capacity(size as usize + 1usize),
            max_position,
            chars_to_render: FallingChar::get_random_chars(rng, size, chars_to_use),
            color_fmt,
            color_lighter_fmt,
            size,
            message,
        }
    }

    /// Get randomly ordered chars to be used in rendering process
    fn get_random_chars(rng: &mut ThreadRng, size: u16, chars_to_use: &str) -> Vec<char> {
        let mut random_chars = chars_to_use.chars().choose_multiple(rng, size as usize);
        // choose_multiple will only chose max of chars_to_use.chars().len() items, but we might want more
        while random_chars.len() < size as usize {
            let amount_left =  (size as usize) - random_chars.len();
            random_chars.extend(chars_to_use.chars().choose_multiple(rng, amount_left));
        }
        random_chars
    }

    /// Should this instance of [FallingChar] be retained or cleaned by [FallerAdder]
    pub fn should_be_retained(&self) -> bool {
        !self.previous_positions.iter().all(|&pp| pp.is_out_of_bounds(&self.max_position))
    }

    /// Render charater and its trail on the `screen`
    pub fn render(&self, rng: &mut ThreadRng, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        if !self.position.is_out_of_bounds(&self.max_position) {
            let char_to_render: char = self.chars_to_render[0];
            write!(
                screen,
                "{}{}{}{}{}",
                cursor::Goto(self.position.x, self.position.y),
                style::Bold,
                self.color_lighter_fmt,
                char_to_render,
                style::NoBold
            )
            .unwrap();
        }

        if !self.previous_positions.is_empty() {
            for (i, pos) in self.previous_positions.iter().enumerate() {
                if !pos.is_out_of_bounds(&self.max_position) {
                    let mut char_to_render: char = self.chars_to_render[i];
                    if i == 0 {
                        char_to_render = self.chars_to_render.choose(rng).unwrap().to_owned();
                        if let Some(message) = self.message.clone() {
                            char_to_render = message.get_char_in_position(pos).unwrap_or(char_to_render);
                        }
                    }
                    write!(
                        screen,
                        "{}{}{}",
                        cursor::Goto(pos.x, pos.y),
                        self.color_fmt,
                        char_to_render
                    )
                    .unwrap();
                }
            }
        }
    }

    /// Advance char position
    pub fn advance(&mut self) {
        if self.previous_positions.len() >= self.size.into() {
            self.previous_positions.remove(0);
        }
        self.previous_positions.push(self.position);
        self.position.y += 1;
    }
}
