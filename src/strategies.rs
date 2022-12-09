use crate::Move;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait Strategy {
    fn choose<'a>(&mut self, moves: &'a [Move]) -> Option<&'a Move>;
}

#[derive(Debug)]
pub struct RandomStrategy {
    rng: ThreadRng,
}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy { rng: thread_rng() }
    }
}

impl Strategy for RandomStrategy {
    fn choose<'a>(&mut self, moves: &'a [Move]) -> Option<&'a Move> {
        moves.choose(&mut self.rng)
    }
}
