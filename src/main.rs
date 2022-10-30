use rand::{prelude::*, distributions};
use termion::{screen::{self, IntoAlternateScreen, AlternateScreen}, color, style, clear, cursor, terminal_size, input::TermRead, event::Key};
use std::{process, thread, time::Duration, io::{self, Write, stdin, Stdout}, cmp::min, cmp::max};
use ctrlc;

#[derive(Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}

struct FallingChar {
    position: Position,
    previous_positions: Vec<Position>,
    max_position: Position,
    char_to_render: Option<char>,
    fg: (&'static str, &'static str),
    size: u16,
}

impl FallingChar {
    fn new(max_x: u16, max_y: u16) -> Self {
        let position = Position { x: thread_rng().gen_range(1..max_x), y: 1 };
        let char_to_render = Some(FallingChar::get_random_char());
        //let char_to_render = None;
        let size = thread_rng().gen_range(max(2, max_y / 3)..max_y);
        Self {
            position,
            previous_positions: Vec::with_capacity(size.into()),
            max_position: Position { x: max_x, y: max_y },
            char_to_render,
            fg: random_fg(),
            size,
        }
    }

    fn get_random_char() -> char {
        thread_rng().sample(distributions::Alphanumeric) as char
    }

    fn out_of_bounds(&self) -> bool {
        self.position.y >= self.max_position.y + self.size || self.position.x > self.max_position.x
    }

    fn render(&self, screen: &mut AlternateScreen<Stdout>) {
        if !self.out_of_bounds() {
            let char_to_render = match self.char_to_render {
                Some(char) => char,
                None => FallingChar::get_random_char(),
            };
            write!(screen, "{}{}{}{}{}",
                cursor::Goto(self.position.x, self.position.y),style::Bold, self.fg.1, char_to_render, style::NoBold)
                .unwrap();

            if self.previous_positions.len() > 0 {
                let mut iterator = self.previous_positions.iter();
                let first_item = iterator.next().unwrap();
                write!(screen, "{}{}{}", cursor::Goto(first_item.x, first_item.y), self.fg.0, ' ').unwrap();

                for pos in iterator {
                    write!(screen, "{}{}{}", cursor::Goto(pos.x, pos.y), self.fg.0, char_to_render).unwrap();
                }
            }
        }
    }

    fn advance(&mut self) {
        if self.previous_positions.len() >= self.size.into() {
            self.previous_positions.remove(0);
        }
        self.previous_positions.push(self.position.clone());
        self.position.y += 1;
    }
}

fn random_fg() -> (&'static str, &'static str) {
    let rand_value = thread_rng().gen_range(2..=8);
    let color: (&str, &str) = match rand_value {
        2 => (color::Red.fg_str(), color::LightRed.fg_str()),
        3 => (color::Green.fg_str(), color::LightGreen.fg_str()),
        4 => (color::Yellow.fg_str(), color::LightYellow.fg_str()),
        5 => (color::Blue.fg_str(), color::LightBlack.fg_str()),
        6 => (color::Magenta.fg_str(), color::LightMagenta.fg_str()),
        7 => (color::Cyan.fg_str(), color::LightCyan.fg_str()),
        8 => (color::White.fg_str(), color::LightWhite.fg_str()),
        _ => (color::Black.fg_str(), color::LightBlack.fg_str()),
    };
    color
}

fn main_loop(falling_chars: &mut Vec<FallingChar>) {
    let mut screen = io::stdout().into_alternate_screen().unwrap();
    for f in falling_chars.iter_mut() {
        f.render(&mut screen);
        f.advance();
    }
    screen.flush().unwrap(); // copy alternate screen to main screen
    thread::sleep(Duration::from_millis(33));
}

#[derive(Debug)]
struct ProbabilityOutOfBoundsError;

fn add_and_retire_fallers(falling_chars: &mut Vec<FallingChar>,
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
