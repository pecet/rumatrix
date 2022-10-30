use rumatrix::rumatrix::*;
use rand::{prelude::*, distributions};
use termion::{screen::{self, IntoAlternateScreen, AlternateScreen}, color, style, clear, cursor, terminal_size, input::TermRead, event::Key};
use std::{process, thread, time::Duration, io::{self, Write, stdin, Stdout}, cmp::min, cmp::max};
use ctrlc;

fn main() {
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
