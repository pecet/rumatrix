use rand::prelude::*;
/// Structure which automatically shuffles it elements when creating and getting element from it,
/// so each time bag is empty, next pass of gets is randomized again.
pub struct RandomVecBag<T: Clone> {
    /// Underlying vector
    vec: Vec<T>,
    /// Pointer to last returned element
    pointer: usize,
    rng: Box<dyn RngCore>,
}

impl<T: Clone> RandomVecBag<T> {
    /// Create new vector bag, use [thread_rng()] as [RngCore] impl
    pub fn new(vec: Vec<T>) -> RandomVecBag<T> {
        Self::with_custom_rng(vec, Box::new(thread_rng()))
    }

    /// Create new vector bag, specify [Vec<T>] as input vector, and boxed [RngCore] impl
    pub fn with_custom_rng(vec: Vec<T>, rng: Box<dyn RngCore>) -> RandomVecBag<T> {
        let mut rvb = Self { 
            vec, 
            pointer: 0,
            rng,
        };
        rvb.vec.shuffle(&mut rvb.rng);
        rvb
    }

    /// Return new element from shuffled bag held in [Vec<T>]
    pub fn get(&mut self) -> Option<&T> {
        if self.vec.is_empty() {
            return None;
        }
        if self.pointer >= self.vec.len() {
            self.pointer = 0;
            return self.vec.get(self.pointer);
        }
        // randomize order BEFORE pointer, so when we return there again bag will be randomized again
        if self.pointer > 0 {
            self.vec
                .swap(self.pointer - 1, self.rng.gen_range(0..self.pointer));
        }
        let element = self.vec.get(self.pointer);
        self.pointer += 1;
        element
    }
}

#[cfg(test)]
mod test {
    use rand::rngs::mock::StepRng;
    use super::*;

    fn test_rng() -> Box<StepRng> {
        Box::new(StepRng::new(12, 7))
    }

    #[test]
    fn new() {
        let bag = RandomVecBag::new(vec![1u8, 1, 1, 1]);
        assert_eq!(bag.vec, vec![1u8, 1, 1, 1]);
    }

    #[test]
    fn new_with_custom_rng() {
        let bag = RandomVecBag::with_custom_rng(
            vec![1u8, 2, 3, 4],
            test_rng()
        );
        assert_eq!(bag.vec, vec![2u8, 3, 4, 1]);
        assert_eq!(bag.pointer, 0);
    }

    #[test]
    fn get() {
        let mut bag = RandomVecBag::with_custom_rng(
            vec![1u8, 2, 3, 4],
            test_rng()
        );
        assert_eq!(bag.vec, vec![2u8, 3, 4, 1]);
        assert_eq!(bag.pointer, 0);
        assert_eq!(bag.get(), Some(&2));
        assert_eq!(bag.pointer, 1);
        assert_eq!(bag.get(), Some(&3));
        assert_eq!(bag.pointer, 2);
        assert_eq!(bag.get(), Some(&4));
        assert_eq!(bag.pointer, 3);
        assert_eq!(bag.vec, vec![3u8, 2, 4, 1]);
        assert_eq!(bag.get(), Some(&1));
        assert_eq!(bag.pointer, 4);
        assert_eq!(bag.vec, vec![4u8, 2, 3, 1]);
        assert_eq!(bag.get(), Some(&4));
        assert_eq!(bag.pointer, 0);
        assert_eq!(bag.vec, vec![4u8, 2, 3, 1]);
        assert_eq!(bag.get(), Some(&4));
        assert_eq!(bag.vec, vec![4u8, 2, 3, 1]);
    }

}