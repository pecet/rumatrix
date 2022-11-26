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
        chars_to_use: &String,
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

    pub fn get_random_chars(rng: &mut ThreadRng, size: u16, chars_to_use: &String) -> Vec<char> {
        let mut random_chars = Vec::with_capacity(size.into());
        for _ in 0..size {
            let char_index = rng.gen_range(0..chars_to_use.len());
            random_chars.push(
                chars_to_use
                    .chars()
                    .nth(char_index)
                    .expect("Char in string out of range"),
            );
        }
        random_chars
    }

    pub fn out_of_bounds(&self) -> bool {
        self.position.y >= self.max_position.y + self.size || self.position.x > self.max_position.x
    }

    pub fn render(&self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
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
                    let char_to_render: char = self.chars_to_render[i];
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
