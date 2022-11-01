use rand::{prelude::*, distributions};
use termion::{screen::AlternateScreen, color, style, cursor};
use std::{io::{Write, Stdout, repeat}, cmp::max, iter::repeat_with};
use crate::position::*;

pub struct FallingChar {
    position: Position,
    previous_positions: Vec<Position>,
    max_position: Position,
    chars_to_render: Vec<char>,
    fg: (&'static str, &'static str),
    size: u16,
    count: u16,
}

impl FallingChar {
    pub fn new(max_x: u16, max_y: u16, fg: i32, chars_to_use: &String) -> Self {
        let position = Position { x: thread_rng().gen_range(1..max_x), y: 1 };
        let size = thread_rng().gen_range(max(2, max_y / 3)..max_y);
        Self {
            position,
            previous_positions: Vec::with_capacity(size.into()),
            max_position: Position { x: max_x, y: max_y },
            chars_to_render: FallingChar::get_random_chars(size, &chars_to_use) ,
            fg: FallingChar::get_color_str(fg),
            size,
            count: 0,
        }
    }

    pub fn get_random_chars(size: u16, chars_to_use: &String) -> Vec<char> {
        let mut random_chars = Vec::with_capacity(size.into());
        for i in 0..size {
            let char_index = thread_rng().gen_range(0..chars_to_use.len());
            random_chars.push(chars_to_use.chars().nth(char_index.into()).expect("Char in string out of range"));
        }
        random_chars
    }

    pub fn random_fg() -> (&'static str, &'static str) {
        let rand_value = thread_rng().gen_range(2..=8);
        FallingChar::get_color_str(rand_value)
    }

    pub fn get_color_str(color: i32) -> (&'static str, &'static str) {
        match color {
            -1 => FallingChar::random_fg(), // will return any of below colors
            2 => (color::Red.fg_str(), color::LightRed.fg_str()),
            3 => (color::Green.fg_str(), color::LightGreen.fg_str()),
            4 => (color::Yellow.fg_str(), color::LightYellow.fg_str()),
            5 => (color::Blue.fg_str(), color::LightBlack.fg_str()),
            6 => (color::Magenta.fg_str(), color::LightMagenta.fg_str()),
            7 => (color::Cyan.fg_str(), color::LightCyan.fg_str()),
            8 => (color::White.fg_str(), color::LightWhite.fg_str()),
            _ => (color::Black.fg_str(), color::LightBlack.fg_str()),
        }
    }

    pub fn out_of_bounds(&self) -> bool {
        self.position.y >= self.max_position.y + self.size || self.position.x > self.max_position.x
    }

    pub fn render(&self, screen: &mut AlternateScreen<Stdout>) {
        if !self.out_of_bounds() {
            let char_to_render: char = self.chars_to_render[0];
            write!(screen, "{}{}{}{}{}",
                cursor::Goto(self.position.x, self.position.y),style::Bold, self.fg.1, char_to_render, style::NoBold)
                .unwrap();

            if !self.previous_positions.is_empty() {
                let mut iterator = self.previous_positions.iter();
                let first_item = iterator.next().unwrap();
                write!(screen, "{}{} ", cursor::Goto(first_item.x, first_item.y), self.fg.0).unwrap();

                for (i, pos) in iterator.enumerate() {
                    let char_to_render: char = self.chars_to_render[i];
                    write!(screen, "{}{}{}", cursor::Goto(pos.x, pos.y), self.fg.0, char_to_render).unwrap();
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