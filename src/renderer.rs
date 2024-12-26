use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::rc::Rc;
use crossterm::{queue, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crate::config::Config;
use crate::state::{Position, State};

pub trait Renderer {
    fn init(&self);
    fn render(&self);
    fn clean_up(&self);
}
pub struct CrossTermRenderer {
    output: RefCell<Stdout>,
    config: Rc<Config>,
    state: Rc<State>,
}

impl CrossTermRenderer {
    pub fn new(state: &Rc<State>, config: &Rc<Config>) -> Self {
        Self {
            output: RefCell::new(stdout()),
            config: Rc::clone(&config),
            state: Rc::clone(&state),
        }
    }
}


impl Renderer for CrossTermRenderer {
    fn init(&self) {
        let mut output = stdout();
        enable_raw_mode().expect("Cannot enable RAW mode for terminal!");
        queue!(output, Hide, EnterAlternateScreen, Clear(ClearType::All));
        output.flush().unwrap();
    }

    fn render(&self) {
        let mut output = stdout();
        let fallers = self.state.get_fallers();
        for faller in fallers.iter() {
            let position: Position<u16> = faller.get_position().cast();
            let symbol = faller.get_symbol();
            queue!(output,
                MoveTo(position.x, position.y),
                SetColors(
                    Colors::new(Color::Green, Color::Reset)
                ),
                Print(symbol)
            ).unwrap();
            output.flush().unwrap();
        }
    }

    fn clean_up(&self) {
        let mut output = stdout();
        queue!(output, Show, LeaveAlternateScreen);
        disable_raw_mode().unwrap();
        output.flush().unwrap();
    }
}

