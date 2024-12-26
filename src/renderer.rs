use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::panic::catch_unwind;
use std::rc::Rc;
use crossterm::{queue, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Attribute, Color, Colors, Print, PrintStyledContent, SetColors};
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum TerminalCell {
    Empty,
    Assigned {
        symbol: char,
        bg: Color,
        fg: Color,
        attribute: Option<Attribute>,
    },
}

struct TerminalCells {
    cell: Vec<TerminalCell>,
    max: Position<u16>,
}

impl TerminalCells {
    pub fn new(max_x: u16, max_y: u16) -> Self {
        let mut cell = Vec::with_capacity(
            ((max_x + 1) * (max_y + 1)) as usize
        );
        for _ in 0..(max_x + 1) * (max_y + 1) {
            cell.push(TerminalCell::Empty);
        }
        Self {
            cell,
            max: Position::new(max_x, max_y)
        }
    }

    fn position_to_1d(&self, position: &Position<u16>) -> Option<u16> {
        if position.x > self.max.x || position.y > self.max.y {
            None
        } else {
            Some((self.max.x + 1) * position.y + position.x)
        }
    }

    pub fn get(&self, position: &Position<u16>) -> &TerminalCell {
        self.position_to_1d(position).and_then(|no| {
            self.cell.get(no as usize)
        }).unwrap()
    }

    pub fn set(&mut self, position: &Position<u16>, value: TerminalCell) {
        if let Some(no) = self.position_to_1d(position) {
            if let Some(cell) = self.cell.get_mut(no as usize) {
                *cell = value;
            }
        }
    }

    /// Compare self to other
    ///
    /// Returns: difference where cells which had different values
    /// are set to other value
    pub fn diff(&self, other: &Self) -> Option<Self> {
        if self.max != other.max {
            None
        } else {
            let mut diff = Self::new(self.max.x, self.max.y);
            for (i, self_cell) in self.cell.iter().enumerate() {
                let other_cell = &other.cell[i];
                if self_cell != other_cell {
                    diff.cell[i] = other_cell.clone();
                }
            }
            Some(diff)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_set_cell() {
        let mut cells = TerminalCells::new(10, 3);
        assert_eq!(
            *cells.get(&Position::new(1, 1)),
            TerminalCell::Empty
        );
        assert_eq!(
            *cells.get(&Position::new(9, 1)),
            TerminalCell::Empty
        );

        let some_value = TerminalCell::Assigned {
            symbol: 'X',
            bg: Color::Reset,
            fg: Color::Red,
            attribute: None,
        };
        cells.set(&Position::new(4,3), some_value.clone());
        assert_eq!(
            *cells.get(&Position::new(4, 3)),
            some_value.clone()
        )
    }
}