pub mod cli_parser;
pub mod falling_char;
pub mod position;
pub mod random_vec_bag;
pub mod faller_adder;
use crate::cli_parser::*;
use crate::faller_adder::FallerAdder;
use crate::falling_char::*;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use position::Position;
use rand::prelude::*;
use random_vec_bag::RandomVecBag;
use termion::AsyncReader;

use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::{async_stdin, clear, color, cursor, screen::ToMainScreen, style, terminal_size};

use clap::Parser;
use std::{
    io::Bytes,
    io::{self, Write},
    process,
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

pub fn main_loop(falling_chars: Rc<RefCell<Vec<FallingChar>>>) {
    let mut falling_chars = falling_chars.borrow_mut();
    let mut screen = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

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
    };
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
    let mut rng = thread_rng();

    ctrlc::set_handler(|| {
        clean_exit();
    })
    .expect("Error handling CTRL+C");

    let default_size = terminal_size().expect("Cannot get terminal size!");
    let default_size = Position { x: default_size.0, y: default_size.1 };

    let size = match (cli.size_x, cli.size_y) {
        (Some(x), Some(y)) => Position { x, y },
        (Some(x), None) => Position { x, y: default_size.y },
        (None, Some(y)) => Position { x: default_size.x, y },
        _ => default_size,
    };

    let color = match cli.color {
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

    let falling_chars = Rc::new(RefCell::new(Vec::with_capacity(no_fallers)));
    let mut vec: Vec<u16> = Vec::with_capacity(usize::from(size.x) * 3);
    // we want unique positions for fallers, but it still looks cool if some fallers fall at the same time at the same position
    for _ in 1..=3 {
        vec.extend(1..=size.x);
    }
    let mut position_bag = RandomVecBag::new(vec);
    let mut stdin = async_stdin().bytes();
    let falling_char_ref1 = Rc::clone(&falling_chars);
    let mut faller_adder: FallerAdder = FallerAdder {
        rng: &mut rng,
        falling_chars: falling_char_ref1,
        max_position: size,
        color_fmt: color.get_color_fmt(),
        color_lighter_fmt: color.get_color_lighter_fmt(),
        max_fallers: no_fallers,
        probability_to_add: 0.22,
        chars_to_use: &chars_to_use,
        positions: &mut position_bag,
    };

    loop {
        let falling_char_ref2 = Rc::clone(&falling_chars);
        handle_keys(&mut stdin);
        main_loop(falling_char_ref2);
        handle_keys(&mut stdin);
        faller_adder.add_and_retire().expect("Cannot add/or retire fallers");
    }
}
