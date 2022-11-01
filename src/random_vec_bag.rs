use rand::prelude::*;

pub struct RandomVecBag<T: Clone> {
    vec: Vec<T>,
    pointer: usize,
}

impl<T: Clone> RandomVecBag<T> {
    pub fn new(vec: Vec<T>) -> RandomVecBag<T> {
        let mut rvb = RandomVecBag {
            vec,
            pointer: 0,
        };
        rvb.vec.shuffle(&mut thread_rng());
        rvb
    }

    pub fn get(&mut self) -> Option<&T> {
        if self.pointer >= self.vec.len() {
            self.pointer = 0;
        }
        let element = self.vec.get(self.pointer);
        self.pointer += 1;
        element
    }
}