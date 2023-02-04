use std::{cell::RefCell, rc::Rc};

use crate::{
    config::Config, falling_char::FallingChar, position::Position, random_vec_bag::RandomVecBag,
};
use rand::prelude::*;

/// Error thrown if `probabilty_to_add` in [FallerAdder] is not `0.0 <= probability <= 1.0`
#[derive(Debug)]
pub struct ProbabilityOutOfBoundsError;

/// Structure holds abstract being which can add new [FallingChar]s or delete existing
pub struct FallerAdder<'a> {
    /// [ThreadRng] to use
    pub rng: &'a mut ThreadRng,
    /// [Vec<FallingChar>] holding all instances of [FallingChar]
    pub falling_chars: Rc<RefCell<Vec<FallingChar<'a>>>>,
    /// Probability to add new falling char on the screen. Value should satisfy `0.0 <= probability <= 1.0` otherwise [ProbabilityOutOfBoundsError] will be generated.
    pub probability_to_add: f64,
    /// Possible x positions of new [FallingChar]s
    pub positions: &'a mut RandomVecBag<u16>,
    /// Configuration to be used when adding new [FallingChar]
    pub config: &'a Config,
}

impl<'a> FallerAdder<'a> {
    /// Adds new [FallingChar]s and retires old ones (e.g. because they are not visible on the screen)
    pub fn add_and_retire(&mut self) -> Result<(), ProbabilityOutOfBoundsError> {
        if !(0.0..=1.0).contains(&self.probability_to_add) {
            return Err(ProbabilityOutOfBoundsError);
        }

        let mut falling_chars = self.falling_chars.borrow_mut();
        // retire old fallers
        falling_chars.retain(|f| f.should_be_retained());

        for _ in falling_chars.len()..*self.config.no_fallers() {
            if self.rng.gen_bool(self.probability_to_add) {
                let position = Position::new(
                    *self
                        .positions
                        .get()
                        .expect("Cannot get random position from bag"),
                    1,
                );
                falling_chars.push(FallingChar::new(
                    self.rng,
                    position,
                    *self.config.screen_size(),
                    self.config.colors(),
                    self.config.chars_to_use(),
                    self.config.message().clone(),
                ))
            }
        }
        Ok(())
    }
}
