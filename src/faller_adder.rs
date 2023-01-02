use std::{rc::Rc, cell::RefCell};

use rand::prelude::*;
use crate::{random_vec_bag::RandomVecBag, falling_char::{FallingChar}, position::Position, config::Config};


#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;
pub struct FallerAdder<'a> {
    pub rng: &'a mut ThreadRng,
    pub falling_chars: Rc<RefCell<Vec<FallingChar>>>,
    pub probability_to_add: f64,
    pub positions: &'a mut RandomVecBag<u16>,
    pub config: &'a Config,
}

impl<'a> FallerAdder<'a> {
    pub fn add_and_retire(&mut self) -> Result<(), ProbabilityOutOfBoundsError> {
        if !(0.0..=1.0).contains(&self.probability_to_add) {
            return Err(ProbabilityOutOfBoundsError);
        }

        let mut falling_chars = self.falling_chars.borrow_mut();
        // retire old fallers
        falling_chars.retain(|f| !f.out_of_bounds());

        for _ in falling_chars.len()..*self.config.no_fallers() {
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
                    *self.config.screen_size(),
                    self.config.color_pair().get_color_fmt(),
                    self.config.color_pair().get_color_lighter_fmt(),
                    self.config.chars_to_use(),
                    self.config.message().clone(),
                ))
            }
        }
        Ok(())
    }
}