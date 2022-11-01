use rand::prelude::*;

pub struct RandomVecBag<'a, T: Clone> {
    current: &'a mut Vec<T>,
    used: &'a mut Vec<T>,
}

impl<'a, T: Clone> RandomVecBag<'a, T> {
    pub fn new(mut vec: &'a mut Vec<T>) -> RandomVecBag<'a, T> {
        let rvb = RandomVecBag {
            current: &mut vec,
            used: &mut Vec::new(),
        };
        rvb.current.shuffle(&mut thread_rng());
        rvb
    }

    pub fn pop(&'a mut self) -> T {
        if self.current.is_empty() {
            self.current = self.used;
            self.used = &mut Vec::new();
        }
        let element = self.current.pop();
        self.used.push(element.clone().unwrap()); // TODO: Remove unwraps, return Option<T>
        element.unwrap()
    }
}