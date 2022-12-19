use rand::prelude::*;

pub struct RandomVecBag<T: Clone> {
    vec: Vec<T>,
    pointer: usize,
}

impl<T: Clone> RandomVecBag<T> {
    pub fn new(vec: Vec<T>) -> RandomVecBag<T> {
        let mut rvb = RandomVecBag { vec, pointer: 0 };
        rvb.vec.shuffle(&mut thread_rng());
        rvb
    }

    pub fn get(&mut self) -> Option<&T> {
        if self.vec.is_empty() {
            return None;
        }
        if self.pointer >= self.vec.len() {
            self.pointer = 0;
        }
        // randomize order BEFORE pointer, so when we return there again bag will be randomize
        if self.pointer > 0 {
            self.vec
                .swap(self.pointer - 1, thread_rng().gen_range(0..self.pointer));
        }
        let element = self.vec.get(self.pointer);
        self.pointer += 1;
        element
    }
}
