use crossterm::style::{Attribute, Color};
use crate::state::Position;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TerminalCell {
    Empty,
    Assigned {
        symbol: char,
        bg: Color,
        fg: Color,
        attribute: Option<Attribute>,
    },
}

#[derive(Debug)]
pub struct TerminalCells {
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

    pub fn empty_sized_as(other: &TerminalCells) -> Self {
        Self::new(other.max.x, other.max.y)
    }

    pub fn from_iter<I>(max_x: u16, max_y: u16, iter: I) -> Self
    where
        I: Iterator<Item = TerminalCell>,
    {
        let mut cells = Self::new(max_x, max_y);
        for (cell, value) in cells.cell.iter_mut().zip(iter) {
            *cell = value;
        }
        cells
    }

    fn position_to_1d(&self, position: &Position<u16>) -> Option<u16> {
        if position.x > self.max.x || position.y > self.max.y {
            None
        } else {
            Some((self.max.x + 1) * position.y + position.x)
        }
    }

    fn position_from_1d(&self, value: u16) -> Option<Position<u16>> {
        if value >= self.cell.len() as u16 {
            None
        } else {
            let x = value % (self.max.x + 1);
            let y = value / (self.max.x + 1);
            Some(Position::new(x, y))
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
    pub fn diff(&self, other: &Self) -> Option<Vec<TerminalDiff>> {
        if self.max != other.max {
            None
        } else {
            let mut diff: Vec<TerminalDiff> = Vec::new();
            for (i, self_cell) in self.cell.iter().enumerate() {
                let other_cell = &other.cell[i];
                if self_cell != other_cell {
                    let position = self.position_from_1d(i as u16).expect("Invalid index!");
                    diff.push(TerminalDiff {
                        position,
                        cell: other_cell.clone(),
                    });
                }
            }
            Some(diff)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TerminalDiff {
    pub position: Position<u16>,
    pub cell: TerminalCell,
}


#[cfg(test)]
mod test {
    use crate::terminal_cell::{TerminalCell, TerminalCells, TerminalDiff};
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

    #[test]
    fn position_to_1d() {
        let mut cells = TerminalCells::new(4, 12);
        assert_eq!(cells.position_to_1d(&Position::new(3,7)).unwrap(), 38);
        let mut cells = TerminalCells::new(5, 2);
        assert_eq!(cells.position_to_1d(&Position::new(2,2)).unwrap(), 14);
        assert!(cells.position_to_1d(&Position::new(5,3)).is_none());
    }

    #[test]
    fn position_from_1d() {
        let mut cells = TerminalCells::new(4, 12);
        assert_eq!(cells.position_from_1d(38).unwrap(), Position::new(3,7));
        let mut cells = TerminalCells::new(5, 2);
        assert_eq!(cells.position_from_1d(14).unwrap(), Position::new(2,2));
        assert!(cells.position_from_1d(101).is_none());
    }

    #[test]
    fn from_iter() {
        fn empty() -> TerminalCell {
            TerminalCell::Empty
        }
        fn filled() -> TerminalCell {
            TerminalCell::Assigned {
                symbol: 'X',
                bg: Color::White,
                fg: Color::Red,
                attribute: None,
            }
        }
        let items = [
            empty(), empty(), filled(),
            filled(), filled(), filled(),
            empty(), filled(), empty(),
            empty(), filled(), filled(),
        ];
        let cells = TerminalCells::from_iter(2, 3, items.into_iter());
        assert_eq!(*cells.get(&Position::new(0, 0)), empty());
        assert_eq!(*cells.get(&Position::new(1, 1)), filled());
        assert_eq!(*cells.get(&Position::new(2, 2)), empty());
        assert_eq!(*cells.get(&Position::new(2, 3)), filled());
        assert_eq!(*cells.get(&Position::new(1, 3)), filled());
    }

    #[test]
    fn diff() {
        fn empty() -> TerminalCell {
            TerminalCell::Empty
        }
        fn g_filled() -> TerminalCell {
            TerminalCell::Assigned {
                symbol: 'G',
                bg: Color::White,
                fg: Color::Red,
                attribute: None,
            }
        }
        fn f_filled() -> TerminalCell {
            TerminalCell::Assigned {
                symbol: 'F',
                bg: Color::Black,
                fg: Color::Cyan,
                attribute: None,
            }
        }
        let cells1 = [
            empty(), g_filled(),
            empty(), g_filled(),
            g_filled(), f_filled(),
        ];
        let cells2 = [
            g_filled(), empty(),
            empty(), g_filled(),
            f_filled(), g_filled(),
        ];
        let cells1 = TerminalCells::from_iter(1, 2, cells1.into_iter());
        let cells2 = TerminalCells::from_iter(1, 2, cells2.into_iter());
        let diff = cells1.diff(&cells2).unwrap();
        let expected = Vec::from_iter([
            TerminalDiff {
                position: Position::new(0, 0),
                cell: g_filled()
            },
            TerminalDiff {
                position: Position::new(1, 0),
                cell: empty(),
            },
            TerminalDiff {
                position: Position::new(0, 2),
                cell: f_filled()
            },
            TerminalDiff {
                position: Position::new(1, 2),
                cell: g_filled()
            },
        ]);
        assert_eq!(diff, expected)
    }
}