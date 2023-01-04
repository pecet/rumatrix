#![allow(missing_docs)]
// only because derive(Getters) give me error about functions derived by it do not have docs
// most likely there is better way to do this

use clap::Parser;
use derive_getters::Getters;
use dyn_clone::{DynClone};
use termion::{terminal_size, color};
use crate::{Position, message::Message};
#[derive(Getters, Clone)]
/// Structure holding shared configuration of the program
pub struct Config {
    /// Current screen size
    screen_size: Position,
    /// Configured [ColorPair] which will be used by the fallers
    color_pair: Box<dyn ColorPair>,
    /// Maximum number of fallers
    no_fallers: usize,
    /// [String] which characters will be used for displaying [FallingChar] and its trail
    chars_to_use: String,
    /// Optional message which will be displayed on the screen
    message: Option<Message>,
}

impl Config {
    /// Get color first 8 colors (1-8) from terminal 16 color pallette via number
    fn get_color(color: i32) -> Box<dyn ColorPair> {
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

    /// Get RGB color
    fn get_rgb_color(r: u8, g: u8, b: u8) -> Box<dyn ColorPair> {
        Box::new(color::Rgb(r, g, b))
    }

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

        let color = match cli.color {
            Some(color_str) => match color_str.parse::<i32>() {
                Ok(color) => Self::get_color(color),
                Err(_) => panic!("Incorrect value for color provided: {}", color_str),
            },
            None => self.color_pair.clone(), // green
        };
        let color = match cli.color_rgb {
            Some(color_str) => {
                let colors_str: Vec<_> = color_str.split(',').collect();
                if colors_str.len() != 3 {
                    panic!("RGB color needs to be specified using following syntax: r,g,b e.g.: 128,128,255");
                }
                let colors_int: Vec<u8> = colors_str.iter().map(|s| s.parse().expect("Cannot convert color value to string")).collect();
                Self::get_rgb_color(colors_int[0], colors_int[1], colors_int[2])
            }
            None => color,
        };
        self.color_pair = color;

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
            color_pair: Box::new(color::Green),
            no_fallers: 50,
            chars_to_use: "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ0123456789!@$%^&*()_+|{}[]<>?!~\\/.,:;".into(),
            message: None,
        }
    }
}


/// Trait to return ANSI formatting string for both head and trail color of [FallingChar]
pub trait ColorPair: DynClone {
    /// Get ANSI formatting string for head color
    fn get_color_fmt(&self) -> String;
    /// Get ANSI formatting string for trail color
    fn get_color_lighter_fmt(&self) -> String;
}

dyn_clone::clone_trait_object!(ColorPair);

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

impl ColorPair for color::Rgb {
    fn get_color_fmt(&self) -> String {
        self.fg_string()
    }

    fn get_color_lighter_fmt(&self) -> String {
        let light_color = (self.0 as u16 + 50, self.1 as u16 + 50, self.2 as u16 + 50);
        let light_color = (
            if light_color.0 > 255 {
                255
            } else {
                light_color.0
            },
            if light_color.1 > 255 {
                255
            } else {
                light_color.1
            },
            if light_color.2 > 255 {
                255
            } else {
                light_color.2
            }
        );
        color::Rgb(light_color.0 as u8, light_color.1 as u8, light_color.2 as u8).fg_string()
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


