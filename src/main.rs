use rand::prelude::*;
use termion::{color, style, clear, cursor, screen, terminal_size};
use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}
#[derive(Debug)]
struct FallingChar {
    position: Position,
    //previous_position: Position,
    max_position: Position,
    char_to_render: char,
    fg: &'static str,
}

impl FallingChar {
    fn new(max_x: u16, max_y: u16) -> Self {
        Self {
            position: Position { x: thread_rng().gen_range(1..max_x), y: 1 },
            max_position: Position { x: max_x, y: max_y },
            char_to_render: '#',
            fg: random_fg(),
        }
    }

    fn out_of_bounds(&self) -> bool {
        self.position.y > self.max_position.y || self.position.x > self.max_position.x
    }

    fn render(&self) {
        if !self.out_of_bounds() {
            print!("{}{}{}", cursor::Goto(self.position.x, self.position.y), self.fg, self.char_to_render);
        }
    }

    fn advance(&mut self) {
        if !self.out_of_bounds() {
            self.position.y += 1;
        }
    }
}

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

fn main_loop(falling_chars: &mut Vec<FallingChar>) {
    for f in falling_chars {
        f.render();
        f.advance();
    }
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let (size_x, size_y) = terminal_size().expect("Cannot get terminal size!");
    let mut falling_chars = vec![FallingChar::new(size_x, size_y), FallingChar::new(size_x, size_y), FallingChar::new(size_x, size_y)];
    print!("{}", clear::All);
    loop {
        main_loop(&mut falling_chars);
    }
}
