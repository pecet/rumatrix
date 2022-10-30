pub mod position;
pub mod falling_char;
use crate::falling_char::*;

use rand::prelude::*;
use termion::{style, clear, cursor, terminal_size, screen::IntoAlternateScreen};
use std::{process, thread, time::Duration, io::{self, Write}};
use ctrlc;

pub fn main_loop(falling_chars: &mut Vec<FallingChar>) {
    let mut screen = io::stdout().into_alternate_screen().unwrap();
    for f in falling_chars.iter_mut() {
        f.render(&mut screen);
        f.advance();
    }
    screen.flush().unwrap(); // copy alternate screen to main screen
    thread::sleep(Duration::from_millis(33));
}

#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;

pub fn add_and_retire_fallers(falling_chars: &mut Vec<FallingChar>,
        max_x: u16, max_y: u16,
        probability_to_add: f64) -> Result<(), ProbabilityOutOfBoundsError> {
    if probability_to_add < 0.0 || probability_to_add > 1.0 {
        return Err(ProbabilityOutOfBoundsError)
    }
    let max_fallers = 140; // hardcoded for now

    // retire old fallers
    falling_chars.retain(|f| !f.out_of_bounds());

    for _ in falling_chars.len()..max_fallers {
        if thread_rng().gen_bool(probability_to_add) {
            falling_chars.push(FallingChar::new(max_x, max_y))
        }
    }
    Ok(())
}

pub fn program_main() {
    ctrlc::set_handler(|| {
        print!("{}{}{}{}", style::Reset, clear::All, cursor::Show, cursor::Goto(1, 1));
        io::stdout().flush().unwrap();
        process::exit(0);
    }).expect("Error handling CTRL+C");

    let (size_x, size_y) = terminal_size().expect("Cannot get terminal size!");
    print!("{}{}{}", clear::All, cursor::Hide, style::Reset);
    io::stdout().flush().unwrap();

    let mut falling_chars = vec![FallingChar::new(size_x, size_y)];
    add_and_retire_fallers(&mut falling_chars, size_x, size_y, 0.5).unwrap();

    loop {
        main_loop(&mut falling_chars);
        add_and_retire_fallers(&mut falling_chars, size_x, size_y, 0.3).unwrap();
    }
}
