pub mod cli_parser;
pub mod falling_char;
pub mod position;
pub mod random_vec_bag;
use crate::cli_parser::*;
use crate::falling_char::*;
use std::io::Read;
use std::io::Stdin;
use std::time::SystemTime;

use position::Position;
use rand::prelude::*;
use random_vec_bag::RandomVecBag;
use termion::AsyncReader;
use termion::event::Event;
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use termion::{
    clear, color, color::Color, cursor, screen::{AlternateScreen, ToAlternateScreen, ToMainScreen}, style, terminal_size, async_stdin
};
use termion::raw::IntoRawMode;

use clap::Parser;
use std::{
    fs,
    io::{self, Write, stdout, stdin},
    process, thread,
    time::Duration,
    cmp::min,
    io::Bytes,
};

#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;

pub fn get_color(color: i32) -> Box<dyn ColorPair> {
    match color {
        2 => Box::new(color::Red),
        3 => Box::new(color::Green),
        4 => Box::new(color::Yellow),
        5 => Box::new(color::Blue),
        6 => Box::new(color::Magenta),
        7 => Box::new(color::Cyan),
        8 => Box::new(color::White),
        _ => Box::new(color::Black),
    }
}

pub fn add_and_retire_fallers(
    falling_chars: &mut Vec<FallingChar>,
    max_x: u16,
    max_y: u16,
    color_fmt: String,
    color_lighter_fmt: String,
    max_fallers: usize,
    probability_to_add: f64,
    chars_to_use: &String,
    positions: &mut RandomVecBag<u16>,
) -> Result<(), ProbabilityOutOfBoundsError> {
    if !(0.0..=1.0).contains(&probability_to_add) {
        return Err(ProbabilityOutOfBoundsError);
    }

    // retire old fallers
    falling_chars.retain(|f| !f.out_of_bounds());

    for _ in falling_chars.len()..max_fallers {
        if thread_rng().gen_bool(probability_to_add) {
            let position = Position {
                x: *positions
                    .get()
                    .expect("Cannot get random position from bag"),
                y: 1,
            };
            falling_chars.push(FallingChar::new(
                position,
                max_x,
                max_y,
                color_fmt.clone(),
                color_lighter_fmt.clone(),
                chars_to_use,
            ))
        }
    }
    Ok(())
}

pub fn handle_keys(stdin: &mut Bytes<AsyncReader>) {
    let key_char = stdin.next();
    if let Some(Ok(b'q')) = key_char {
        clean_exit();
    }
}

pub fn clean_exit() {
    print!(
        "{}{}{}{}",
        style::Reset,
        clear::All,
        cursor::Show,
        cursor::Goto(1, 1)
    );
    io::stdout().flush().unwrap();
    process::exit(0);
}

pub fn main_loop(falling_chars: &mut [FallingChar]) {
    let mut screen = io::stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

    write!(screen, "{}", ToMainScreen).unwrap();

    for f in falling_chars.iter_mut() {
        f.render(&mut screen);
        f.advance();
    }
    screen.flush().unwrap(); // flush alternate screen
    drop(screen); // copy alternate screen to main screen
}

pub trait ColorPair {
    fn get_color_fmt(&self) -> String;
    fn get_color_lighter_fmt(&self) -> String;
}

macro_rules! add_color_pair {
    ($name: ident, $light_name: ident) => {
        impl ColorPair for color::$name {
            fn get_color_fmt(&self) -> String {
                self.fg_str().to_string()
            }

            fn get_color_lighter_fmt(&self) -> String {
                color::$light_name.fg_str().to_string()
            }
        }
    }
}

add_color_pair!(Black, LightBlack);
add_color_pair!(Red, LightRed);
add_color_pair!(Green, LightGreen);
add_color_pair!(Yellow, LightYellow);
add_color_pair!(Blue, LightBlue);
add_color_pair!(Magenta, LightMagenta);
add_color_pair!(Cyan, LightCyan);
add_color_pair!(White, LightWhite);

pub fn program_main() {
    let cli = Cli::parse();

    ctrlc::set_handler(|| {
        clean_exit();
    })
    .expect("Error handling CTRL+C");

    let default_size = terminal_size().expect("Cannot get terminal size!");

    let (size_x, size_y) = match (cli.size_x, cli.size_y) {
        (Some(x), Some(y)) => (x, y),
        (Some(x), None) => (x, default_size.1),
        (None, Some(y)) => (default_size.0, y),
        _ => default_size,
    };

    let mut color: Box<dyn ColorPair>;

    color = match cli.color {
        Some(color_str) => match color_str.parse::<i32>() {
            Ok(color) => get_color(color),
            Err(_) => panic!("Incorrect value for color provided: {}", color_str),
        },
        None => get_color(3), // green
    };

    let no_fallers = match cli.no_fallers {
        Some(no) => match no {
            0 => 1,
            _ => no,
        },
        None => 100,
    };

    let chars_to_use = match cli.chars_to_use {
        Some(str) => str,
        None => {
            "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789!@$%^&*()_+|{}[]<>?!~"
                .into()
        }
    };

    print!("{}{}{}", clear::All, cursor::Hide, style::Reset);
    io::stdout().flush().unwrap();

    let mut falling_chars: Vec<FallingChar> = Vec::with_capacity(no_fallers);
    let mut vec: Vec<u16> = Vec::with_capacity(usize::from(size_x) * 3);
    // we want unique positions for fallers, but it still looks cool if some fallers fall at the same time at the same position
    for _ in 1..=3 {
        vec.extend(1..=size_x);
    }
    let mut position_bag = RandomVecBag::new(vec);

    color::Black.fg_str();
    color::Rgb(12,12,12).fg_string();
    let mut stdin = async_stdin().bytes();

    loop {
        handle_keys(&mut stdin);
        main_loop(&mut falling_chars);
        handle_keys(&mut stdin);
        add_and_retire_fallers(
            &mut falling_chars,
            size_x,
            size_y,
            color.get_color_fmt(),
            color.get_color_lighter_fmt(),
            no_fallers,
            0.22,
            &chars_to_use,
            &mut position_bag,
        )
        .expect("Cannot add/or retire fallers");
    }
}
