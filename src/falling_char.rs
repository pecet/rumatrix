use crate::position::*;
use rand::prelude::*;
use std::{
    cmp::max,
    io::{Stdout, Write},
};
use termion::raw::RawTerminal;
use termion::{cursor, screen::AlternateScreen, style};

pub struct FallingChar {
    position: Position,
    previous_positions: Vec<Position>,
    max_position: Position,
    chars_to_render: Vec<char>,
    color_fmt: String,
    color_lighter_fmt: String,
    size: u16,
}

impl FallingChar {
    pub fn new(
        rng: &mut ThreadRng,
        position: Position,
        max_position: Position,
        color_fmt: String,
        color_lighter_fmt: String,
        chars_to_use: &str,
    ) -> Self {
        let size = rng.gen_range(max(2, max_position.y / 3)..max_position.y);
        Self {
            position,
            previous_positions: Vec::with_capacity(size.into()),
            max_position,
            chars_to_render: FallingChar::get_random_chars(rng, size, chars_to_use),
            color_fmt,
            color_lighter_fmt,
            size,
        }
    }

    pub fn get_random_chars(rng: &mut ThreadRng, size: u16, chars_to_use: &str) -> Vec<char> {
        let mut random_chars = chars_to_use.chars().choose_multiple(rng, size as usize);
        // choose_multiple will only chose max of chars_to_use.chars().len() items, but we might want more
        while random_chars.len() < size as usize {
            let amount_left =  (size as usize) - random_chars.len();
            random_chars.extend(chars_to_use.chars().choose_multiple(rng, amount_left));
        }
        random_chars
    }

    pub fn out_of_bounds(&self) -> bool {
        self.position.y >= self.max_position.y + self.size || self.position.x > self.max_position.x
    }

    pub fn render(&self, rng: &mut ThreadRng, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        let hardcoded_msg = "     The Matrix has you...     ".to_owned();
        let hardcoded_position = Position {x: 100, y: 25};
        let mut already_placed = vec![false; hardcoded_msg.len()];

        if !self.out_of_bounds() {
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

            if !self.previous_positions.is_empty() {
                for (i, pos) in self.previous_positions.iter().enumerate() {
                    let char_to_render: char = if i == 0 {
                        if pos.y == hardcoded_position.y && pos.x >= hardcoded_position.x && pos.x < hardcoded_position.x + hardcoded_msg.len() as u16 {
                            let nth = pos.x - hardcoded_position.x;
                            if !already_placed[nth as usize] {
                                already_placed[nth as usize] = true;
                                hardcoded_msg.chars().nth(nth as usize).unwrap()
                            } else {
                                self.chars_to_render.choose(rng).unwrap().to_owned()
                            }
                        } else {
                            self.chars_to_render.choose(rng).unwrap().to_owned()
                        }
                    } else {
                        self.chars_to_render[i]
                    };

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

    pub fn advance(&mut self) {
        if self.previous_positions.len() >= self.size.into() {
            self.previous_positions.remove(0);
        }
        self.previous_positions.push(self.position);
        self.position.y += 1;
    }
}
