#![allow(missing_docs)]
// only because derive(Getters) give me error about functions derived by it do not have docs
// most likely there is better way to do this

use clap::Parser;
use derive_getters::Getters;
use serde::{Serialize, Deserialize};
use termion::{terminal_size, color};
use crate::{Position, message::Message, colors::{Colors, Color}};
#[derive(Getters, Clone, Serialize, Deserialize)]
/// Structure holding shared configuration of the program
pub struct Config {
    /// Current screen size
    screen_size: Position,
    /// Configured [ColorPair] which will be used by the fallers
    colors: Colors,
    /// Maximum number of fallers
    no_fallers: usize,
    /// [String] which characters will be used for displaying [FallingChar] and its trail
    chars_to_use: String,
    /// Optional message which will be displayed on the screen
    message: Option<Message>,
}



impl Config {
    /// Create new [Config] instance with default values
    pub fn new_with_defaults() -> Self {
        Default::default()
    }

    /// Parse [Config] from [Cli] (via clap).
    ///
    /// Overwrite defaults with parameters from Cli, or do not if parameter is not present.
    pub fn parse_cli(&mut self) {
        let cli = Cli::parse();

        let size = match (cli.size_x, cli.size_y) {
            (Some(x), Some(y)) => Position { x, y },
            (Some(x), None) => Position { x, ..self.screen_size },
            (None, Some(y)) => Position { y, ..self.screen_size },
            _ => self.screen_size,
        };
        self.screen_size = size;

        let color_trail = match cli.color {
            Some(color_str) => match color_str.parse::<u8>() {
                Ok(color) => Color::Palette(color),
                Err(_) => panic!("Incorrect value for color provided: {}", color_str),
            },
            None => self.colors.trail.clone(),
        };
        let color_trail = match cli.color_rgb {
            Some(color_str) => {
                let colors_str: Vec<_> = color_str.split(',').collect();
                if colors_str.len() != 3 {
                    panic!("RGB color needs to be specified using following syntax: r,g,b e.g.: 128,128,255");
                }
                let colors_int: Vec<u8> = colors_str.iter().map(|s| s.parse().expect("Cannot convert color value to string")).collect();
                Color::rgb_from_vec(colors_int)
            }
            None => color_trail,
        };
        self.colors.head = color_trail.get_alternate_color();
        self.colors.trail = color_trail;

        let no_fallers = match cli.no_fallers {
            Some(no) => match no {
                0 => 1,
                _ => no,
            },
            None => self.no_fallers,
        };
        self.no_fallers = no_fallers;

        let chars_to_use = match cli.chars_to_use {
            Some(str) => str,
            None => self.chars_to_use.clone(),
        };
        self.chars_to_use = chars_to_use;

        let message = cli.message.clone().map(|message| {
                if message.len() as u16 > size.x {
                    panic!("Message size ({}) is bigger than maximum value of screen x coordinate ({})!", message.len(), size.x);
                }
                Message {
                    position: Position {
                        x: (size.x - message.len() as u16) / 2,
                        y: size.y / 2,
                    },
                    text: message,
                }
            }
        );
        self.message = message;
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_size = terminal_size().expect("Cannot get terminal size!");
        Self {
            screen_size: Position { x: default_size.0, y: default_size.1 },
            colors: Colors {
                trail: Color::Palette(2),
                head: Color::Palette(10),
            },
            no_fallers: 50,
            chars_to_use: "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789!@$%^&*()_+|{}[]<>?!~\\/.,:;".into(),
            message: None,
        }
    }
}


#[derive(Parser)]
#[command(version)]
#[command(name = "ruMatrix")]
#[command(author = "Piotr Czarny")]
#[command(about = "cmatrix inspired program but in Rust", long_about = None)]
struct Cli {
    /// Force width (x) of the screen
    #[arg(long, short = 'x')]
    size_x: Option<u16>,

    /// Force height (y) of the screen
    #[arg(long, short = 'y')]
    size_y: Option<u16>,

    /// Select color (1-8 inclusive) of fallers or 'rnd' for random
    #[arg(long, short = 'c')]
    color: Option<String>,

    /// Select color (r,g,b; 0-255 each) e.g. 50,50,255
    #[arg(long, short = 'C')]
    color_rgb: Option<String>,

    /// Number of fallers
    #[arg(long, short = 'n')]
    no_fallers: Option<usize>,

    /// Chars to use, if not specified use default list
    #[arg(long, short = 'u')]
    chars_to_use: Option<String>,

    /// Message to show on the screen (default: no message)
    #[arg(long = "msg", short = 'm')]
    message: Option<String>,
}


