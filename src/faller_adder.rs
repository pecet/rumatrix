use std::{rc::Rc, cell::RefCell};

use rand::prelude::*;
use crate::{random_vec_bag::RandomVecBag, falling_char::{FallingChar}, position::Position, message::Message};


#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;
pub struct FallerAdder<'a> {
    pub rng: &'a mut ThreadRng,
    pub falling_chars: Rc<RefCell<Vec<FallingChar>>>,
    pub max_position: Position,
    pub color_fmt: String,
    pub color_lighter_fmt: String,
    pub max_fallers: usize,
    pub probability_to_add: f64,
    pub chars_to_use: &'a String,
    pub positions: &'a mut RandomVecBag<u16>,
    pub message: Option<Message>,
}

impl<'a> FallerAdder<'a> {
    pub fn add_and_retire(&mut self) -> Result<(), ProbabilityOutOfBoundsError> {
        if !(0.0..=1.0).contains(&self.probability_to_add) {
            return Err(ProbabilityOutOfBoundsError);
        }

        let mut falling_chars = self.falling_chars.borrow_mut();
        // retire old fallers
        falling_chars.retain(|f| !f.out_of_bounds());

        for _ in falling_chars.len()..self.max_fallers {
            if self.rng.gen_bool(self.probability_to_add) {
                let position = Position {
                    x: *self.positions
                        .get()
                        .expect("Cannot get random position from bag"),
                    y: 1,
                };
                falling_chars.push(FallingChar::new(
                    self.rng,
                    position,
                    self.max_position,
                    self.color_fmt.clone(),
                    self.color_lighter_fmt.clone(),
                    self.chars_to_use,
                    self.message.clone(),
                ))
            }
        }
        Ok(())
    }
}