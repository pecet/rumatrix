use rand::prelude::*;

/// Structure which automatically shuffles it elements when creating and getting element from it,
/// so each time bag is empty, next pass of gets is randomized again.
pub struct RandomVecBag<T: Clone> {
    /// Underlying vector
    vec: Vec<T>,
    /// Pointer to last returned element
    pointer: usize,
}

impl<T: Clone> RandomVecBag<T> {
    /// Create new vector bag, specify [Vec<T>] as input
    pub fn new(vec: Vec<T>) -> RandomVecBag<T> {
        let mut rvb = RandomVecBag { vec, pointer: 0 };
        rvb.vec.shuffle(&mut thread_rng());
        rvb
    }

    /// Return new element from shuffled bag held in [Vec<T>]
    pub fn get(&mut self) -> Option<&T> {
        if self.vec.is_empty() {
            return None;
        }
        if self.pointer >= self.vec.len() {
            self.pointer = 0;
        }
        // randomize order BEFORE pointer, so when we return there again bag will be randomized again
        if self.pointer > 0 {
            self.vec
                .swap(self.pointer - 1, thread_rng().gen_range(0..self.pointer));
        }
        let element = self.vec.get(self.pointer);
        self.pointer += 1;
        element
    }
}
