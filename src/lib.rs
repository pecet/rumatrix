pub mod falling_char;
pub mod position;
pub mod random_vec_bag;
pub mod faller_adder;
pub mod message;
pub mod config;
use crate::config::Config;
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
use termion::{async_stdin, clear, cursor, screen::ToMainScreen, style};


use std::{
    io::Bytes,
    io::{self, Write},
    process,
};

#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;

pub fn handle_keys(stdin: &mut Bytes<AsyncReader>) {
    let key_char = stdin.next();
    if let Some(Ok(b'q')) = key_char {
        clean_exit();
    }
    if let Some(Ok(b'c')) = key_char {
        print!(
            "{}{}",
            style::Reset,
            clear::All
        )
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
        f.render(&mut thread_rng(), &mut screen);
        f.advance();
    }
    screen.flush().unwrap(); // flush alternate screen
    drop(screen); // copy alternate screen to main screen
    std::thread::sleep(std::time::Duration::from_millis(2));
}

pub fn program_main() {
    let mut rng = thread_rng();
    let mut config = Config::new_with_defaults();
    config.parse_cli();

    ctrlc::set_handler(|| {
        clean_exit();
    })
    .expect("Error handling CTRL+C");

    print!("{}{}{}", clear::All, cursor::Hide, style::Reset);
    io::stdout().flush().unwrap();

    let falling_chars = Rc::new(RefCell::new(Vec::with_capacity(*config.no_fallers())));
    let mut vec: Vec<u16> = Vec::with_capacity(usize::from(config.screen_size().x) * 3);
    // we want unique positions for fallers, but it still looks cool if some fallers fall at the same time at the same position
    for _ in 1..=3 {
        vec.extend(1..=config.screen_size().x);
    }
    let mut position_bag = RandomVecBag::new(vec);
    let mut stdin = async_stdin().bytes();
    let falling_char_ref1 = Rc::clone(&falling_chars);
    let mut faller_adder = FallerAdder {
        rng: &mut rng,
        falling_chars: falling_char_ref1,
        probability_to_add: 0.22,
        positions: &mut position_bag,
        config: &config,
    };

    loop {
        let falling_char_ref2 = Rc::clone(&falling_chars);
        handle_keys(&mut stdin);
        main_loop(falling_char_ref2);
        handle_keys(&mut stdin);
        faller_adder.add_and_retire().expect("Cannot add/or retire fallers");
    }
}
