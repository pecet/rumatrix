use rand::prelude::*;
use termion::{color, style, clear, cursor, screen, terminal_size};

fn random_fg() -> &'static str {
    let rand_value = thread_rng().gen_range(2..=8);
    let color: &str = match rand_value {
        2 => color::Red.fg_str(),
        3 => color::Green.fg_str(),
        4 => color::Yellow.fg_str(),
        5 => color::Blue.fg_str(),
        6 => color::Magenta.fg_str(),
        7 => color::Cyan.fg_str(),
        8 => color::White.fg_str(),        
        _ => color::Black.fg_str(),
    };
    color
}

fn main() {
    let (size_x, size_y) = terminal_size().expect("Cannot get terminal size!");
    print!("{}", clear::All);
    
    for x in 1..=size_x {
        for y in 1..=size_y {
            print!("{}", cursor::Goto(x, y));
            print!("{}#", random_fg());
        }
    }

}
