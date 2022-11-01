use rand::prelude::*;

pub struct RandomVecBag<T: Clone> {
    current: Vec<T>,
    used: Vec<T>,
}

impl<T: Clone> RandomVecBag<T> {
    pub fn new(vec: Vec<T>) -> RandomVecBag<T> {
        let mut rvb = RandomVecBag {
            current: vec,
            used: Vec::new(),
        };
        rvb.current.shuffle(&mut thread_rng());
        rvb
    }

    pub fn pop(&mut self) -> T {
        if self.current.is_empty() {
            self.current = self.used.clone(); // TODO: is it possible without cloning?
            self.used = Vec::new();
        }
        let element = self.current.pop();
        self.used.push(element.clone().unwrap());
        element.unwrap()
    }
}