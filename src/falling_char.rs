use crate::position::*;
use rand::prelude::*;
use std::{
    cmp::max,
    io::{Stdout, Write},
};
use termion::{color, cursor, screen::{AlternateScreen, ToAlternateScreen, ToMainScreen}, style};
use termion::raw::RawTerminal;

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
        position: Position,
        max_x: u16,
        max_y: u16,
        color_fmt: String,
        color_lighter_fmt: String,
        chars_to_use: &String,
    ) -> Self {
        let size = thread_rng().gen_range(max(2, max_y / 3)..max_y);
        Self {
            position,
            previous_positions: Vec::with_capacity(size.into()),
            max_position: Position { x: max_x, y: max_y },
            chars_to_render: FallingChar::get_random_chars(size, chars_to_use),
            color_fmt,
            color_lighter_fmt,
            size,
        }
    }

    pub fn get_random_chars(size: u16, chars_to_use: &String) -> Vec<char> {
        let mut random_chars = Vec::with_capacity(size.into());
        for _ in 0..size {
            let char_index = thread_rng().gen_range(0..chars_to_use.len());
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
