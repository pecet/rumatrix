pub mod position;
pub mod falling_char;
pub mod cli_parser;
use crate::falling_char::*;
use crate::cli_parser::*;

use rand::prelude::*;
use termion::{style, clear, cursor, terminal_size, screen::IntoAlternateScreen};

use std::{process, thread, time::Duration, io::{self, Write}};
use clap::Parser;


#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;

pub fn add_and_retire_fallers (
        falling_chars: &mut Vec<FallingChar>,
        max_x: u16,
        max_y: u16,
        color: i32,
        max_fallers: usize,
        probability_to_add: f64,
        chars_to_use: &String,
    ) -> Result<(), ProbabilityOutOfBoundsError> {

    if !(0.0..=1.0).contains(&probability_to_add) {
        return Err(ProbabilityOutOfBoundsError)
    }

    // retire old fallers
    falling_chars.retain(|f| !f.out_of_bounds());

    for _ in falling_chars.len()..max_fallers {
        if thread_rng().gen_bool(probability_to_add) {
            falling_chars.push(FallingChar::new(max_x, max_y, color, chars_to_use))
        }
    }
    Ok(())
}

pub fn main_loop(falling_chars: &mut [FallingChar]) {
    let mut screen = io::stdout().into_alternate_screen().unwrap();
    for f in falling_chars.iter_mut() {
        f.render(&mut screen);
        f.advance();
    }
    screen.flush().unwrap(); // copy alternate screen to main screen
    thread::sleep(Duration::from_millis(33));
}

pub fn program_main() {
    let cli = Cli::parse();

    ctrlc::set_handler(|| {
        print!("{}{}{}{}", style::Reset, clear::All, cursor::Show, cursor::Goto(1, 1));
        io::stdout().flush().unwrap();
        process::exit(0);
    }).expect("Error handling CTRL+C");

    let default_size = terminal_size().expect("Cannot get terminal size!");

    let (size_x, size_y) = match (cli.size_x, cli.size_y) {
        (Some(x), Some(y)) => (x, y),
        (Some(x), None) => (x, default_size.1),
        (None, Some(y)) => (default_size.0, y),
        _ => default_size,
    };

    let color = match cli.color {
        Some(color_str) => {
            match color_str.parse::<i32>() {
                Ok(color) => color,
                Err(_) => {
                    if color_str == "rnd" {
                        -1
                    } else {
                        panic!("Incorrect value for color provided: {}", color_str)
                    }
                }
            }
        }
        None => 3, // green
    };
    if color != -1 && !(1..=8).contains(&color) {
        panic!("Incorrect value for color provided: {}", color)
    }

    let no_fallers = match cli.no_fallers {
        Some(no) => match no {
            0 => 1,
            _ => no,
        }
        None => 100,
    };

    let chars_to_use = match cli.chars_to_use {
        Some(str) => str,
        None => "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ01234567890!@$%^&*()_+|{}[]<>?!~".into(),
    };

    print!("{}{}{}", clear::All, cursor::Hide, style::Reset);
    io::stdout().flush().unwrap();

    let mut falling_chars: Vec<FallingChar> = Vec::with_capacity(no_fallers);

    loop {
        main_loop(&mut falling_chars);
        add_and_retire_fallers(&mut falling_chars, size_x, size_y, color, no_fallers, 0.22, &chars_to_use).unwrap();
    }
}
