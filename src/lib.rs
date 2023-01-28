//! Library used for ruMatrix
#![warn(missing_docs)]

/// [FallingChar] module
pub mod falling_char;
/// [Position] module
pub mod position;
/// [RandomVecBag] module
pub mod random_vec_bag;
/// [FallerAdder] module
pub mod faller_adder;
/// [Message] module
pub mod message;
/// [Config] module
pub mod config;
/// [Colors] and [Color] module
pub mod colors;
use crate::config::{Config, Cli};
use crate::faller_adder::FallerAdder;
use crate::falling_char::*;

use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

use clap::Parser;
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

// Easiest way to have this parametrized via cli IMHO
// TODO: Find better way
static INCLUDE_DEFAULTS_IN_SERIALIZATION: AtomicBool = AtomicBool::new(false);

/// Handle keyboard input
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

/// Executed when exiting program, clears screen and shows cursor again
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

/// Main loop of the program
pub fn main_loop(falling_chars: Rc<RefCell<Vec<FallingChar>>>) {
    let start_time = SystemTime::now();
    let mut falling_chars = falling_chars.borrow_mut();
    let mut screen = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    write!(screen, "{ToMainScreen}").unwrap();

    for f in falling_chars.iter_mut() {
        f.render(&mut thread_rng(), &mut screen);
        f.advance();
    }
    screen.flush().unwrap(); // flush alternate screen
    let time_elapsed = SystemTime::now().duration_since(start_time).expect("Cannot get elapsed time").as_millis();
    let time_to_sleep = if time_elapsed < 33 { // 1/30 second ~= 33 ms
        33 - time_elapsed as u64
    } else {
        1
    };
    drop(screen); // copy alternate screen to main screen
    std::thread::sleep(std::time::Duration::from_millis(time_to_sleep));
}

/// Main function of the program
pub fn program_main() {
    let mut rng = thread_rng();
    let cli = Cli::parse();

    let mut config = match cli.config_file {
        Some(config_file) => {
            let config_string = fs::read_to_string(config_file).expect("Cannot read config file, make sure that specified path to it is correct.");
            serde_yaml::from_str(&config_string).expect("Incorrect config file contents.")
        }
        None => Config::new_with_defaults(),
    };
    config.parse_cli();

    if cli.print_full_config {
        println!("# Current config YAML, includes:");
        println!("#   Explicit defaults (some values e.g.: screen size might be computed at runtime)");
        println!("#   Overwritten by settings loaded from config file (if any)");
        println!("#   Overwritten by settings loaded from command line (if any)");
        INCLUDE_DEFAULTS_IN_SERIALIZATION.store(true, Ordering::SeqCst);
        println!("{}", serde_yaml::to_string(&config).expect("Cannot serialize current config!"));
        process::exit(0);
    } else if cli.print_config {
        println!("# Current config YAML, includes:");
        println!("#   Settings loaded from config file (if any)");
        println!("#   Overwritten by settings loaded from command line (if any)");
        INCLUDE_DEFAULTS_IN_SERIALIZATION.store(false, Ordering::SeqCst);
        println!("{}", serde_yaml::to_string(&config).expect("Cannot serialize current config!"));
        process::exit(0);
    }

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
