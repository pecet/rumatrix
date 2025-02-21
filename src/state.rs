use crate::config::Config;
use crossterm::cursor::position;
use num_traits::{Num, NumCast, NumOps, ToPrimitive};
use rand::rng;
use rand::seq::IndexedRandom;
use serde_yml::modules::error::Pos;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

type DeltaType = f64;
pub struct Faller {
    position: Position<DeltaType>,
    symbol: char,
}

impl Faller {
    pub fn new(position: Position<DeltaType>, symbol: char) -> Self {
        Self { position, symbol }
    }

    pub fn get_position(&self) -> &Position<DeltaType> {
        &self.position
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }
}

impl Default for Faller {
    fn default() -> Self {
        let x = Position::new(0.0, 0.0);
        Self::new(Position::new(0.0, 0.0), 'X')
    }
}

#[derive(PartialEq, Debug)]
pub struct Position<N: Num> {
    pub x: N,
    pub y: N,
}

impl<N: Num + Copy + ToPrimitive> Position<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn as_tuple<T>(&self) -> (T, T)
    where
        T: NumCast + Default + Copy,
    {
        (
            T::from(self.x).unwrap_or(T::default()),
            T::from(self.y).unwrap_or(T::default()),
        )
    }

    pub fn cast<T: Num + NumCast + Copy + Default>(&self) -> Position<T> {
        Position {
            x: T::from(self.x).unwrap_or(T::default()),
            y: T::from(self.y).unwrap_or(T::default()),
        }
    }
}
pub struct State {
    config: Rc<Config>,
    fallers: RefCell<Vec<Faller>>,
}

impl State {
    pub fn new(config: &Rc<Config>) -> Self {
        let config = Rc::clone(config);
        let mut fallers = Vec::new();
        for i in 0..10 {
            fallers.push(Faller::new(
                Position::new(i as DeltaType * 3.0, 0.0),
                config.symbols.choose(&mut rng()).unwrap().clone(),
            ))
        }
        Self {
            config,
            fallers: RefCell::new(fallers),
        }
    }
    pub fn reset(&self) {
        self.fallers.borrow_mut().clear();
    }
    pub fn advance(&self, delta_time: DeltaType) {
        let config = Rc::clone(&self.config);
        for faller in self.fallers.borrow_mut().iter_mut() {
            faller.position.y += 2.5 * delta_time;
            if faller.position.y > config.size_y.unwrap().into() {
                faller.position.y = 0.0;
            }
        }
    }
    pub fn get_fallers(&self) -> Ref<Vec<Faller>> {
        self.fallers.borrow()
    }
}
