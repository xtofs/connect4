use connect4::{Board, Move, Percent, Player};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::{env::args, time::SystemTime};

trait Strategy {
    fn choose<'a>(&mut self, moves: &'a [Move]) -> Option<&'a Move>;
}

#[derive(Debug)]
struct RandomStrategy {
    rng: ThreadRng,
}

impl RandomStrategy {
    fn new() -> Self {
        RandomStrategy { rng: thread_rng() }
    }
}

impl Strategy for RandomStrategy {
    fn choose<'a>(&mut self, moves: &'a [Move]) -> Option<&'a Move> {
        moves.choose(&mut self.rng)
    }
}

fn main() {
    let mut rng = thread_rng();
    let (mut x_count, mut tie_count, mut o_count) = (0, 0, 0);

    let n: usize = args().nth(1).map(|n| n.parse().unwrap()).unwrap_or(1000);

    println!("running: {} games", n);
    let start = SystemTime::now();
    for _ in 0..n {
        let mut board = Board::default();

        let mut strategies = [RandomStrategy::new(), RandomStrategy::new()];

        loop {
            match board.state() {
                Some(state) => {
                    match state {
                        Some(Player::X) => x_count += 1,
                        Some(Player::O) => o_count += 1,
                        None => tie_count += 1,
                    }
                    break;
                }
                _ => {}
            }
            let strategy = &mut strategies[board.turn as usize];
            let moves = board.moves();
            if let Some(mv) = strategy.choose(&moves) {
                board = board.play(&mv);
            } else {
                break;
            }
        }
    }
    let elapsed = start.elapsed().unwrap();
    println!("elapsed: {:.1?}", elapsed);

    let x = Percent::from_total(x_count, n);
    let tie = Percent::from_total(tie_count, n);
    let o = Percent::from_total(o_count, n);
    println!("stats: X={} tie={} O={}", x, tie, o);
}
