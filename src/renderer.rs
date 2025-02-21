use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::rc::Rc;
use crossterm::{queue, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Colors, Print, SetAttribute, SetColors};
use crossterm::style::Color::Reset;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crate::config::Config;
use crate::state::{Position, State};
use crate::terminal_cell::{TerminalCell, TerminalCells};

pub trait Renderer {
    fn init(&self);
    fn render(&self);
    fn clean_up(&self);
}
pub struct CrossTermRenderer {
    config: Rc<Config>,
    state: Rc<State>,
    cell_buffer: RefCell<TerminalCells>,
}

impl CrossTermRenderer {
    pub fn new(state: &Rc<State>, config: &Rc<Config>) -> Self {
        Self {
            config: Rc::clone(&config),
            state: Rc::clone(&state),
            cell_buffer: RefCell::new(TerminalCells::new(
                config.size_x.unwrap(), config.size_y.unwrap()
            ))
        }
    }
}

impl CrossTermRenderer {
    fn create_buffer(&self) -> TerminalCells {
        let mut buffer = TerminalCells::empty_sized_as(&self.cell_buffer.borrow());
        let fallers = self.state.get_fallers();
        for faller in fallers.iter() {
            let position: Position<u16> = faller.get_position().cast();
            let symbol = faller.get_symbol();
            buffer.set(&position, TerminalCell::Assigned {
                symbol,
                bg: Color::Reset,
                fg: Color::Green,
                attribute: None,
            })
        }
        buffer
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
        let old_buffer = &self.cell_buffer;
        let new_buffer = self.create_buffer();
        let diff = old_buffer.borrow().diff(&new_buffer);
        if let Some(diff) = diff {
            for d in diff {
                queue!(output, MoveTo(d.position.x, d.position.y));
                match d.cell {
                    TerminalCell::Empty => {
                        queue!(output,
                            SetColors(Colors::new(Reset, Reset)),
                            Print(' ')
                        );
                    }
                    TerminalCell::Assigned { symbol, bg, fg, attribute} => {
                        queue!(output, SetColors(Colors::new(fg, bg)));
                        if let Some(attribute) = attribute {
                            queue!(output, SetAttribute(attribute));
                        }
                        queue!(output, Print(symbol));
                    }
                }
            }
            output.flush().unwrap();
            let m = &self.cell_buffer.replace(new_buffer);
        }
    }

    fn clean_up(&self) {
        let mut output = stdout();
        queue!(output, Show, LeaveAlternateScreen);
        disable_raw_mode().unwrap();
        output.flush().unwrap();
    }
}

