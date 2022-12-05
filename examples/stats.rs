use std::{env::args, fmt::Display};

use connect4::{Board, Color};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let (mut red, mut tie, mut green) = (0, 0, 0);

    let n: usize = args().nth(1).map(|n| n.parse().unwrap()).unwrap_or(1000);

    println!("running {n} games");
    for _ in 0..n {
        let mut board = Board::default();

        loop {
            match board.state() {
                Some(state) => {
                    match state {
                        Some(Color::Red) => red += 1,
                        Some(Color::Green) => green += 1,
                        None => tie += 1,
                    }
                    break;
                }
                _ => {}
            }

            let moves = board.moves();
            if let Some(mv) = moves.choose(&mut rng) {
                board = board.play(&mv);
            } else {
                break;
            }
        }
    }
    let red = Percent::from_total(red, n);
    let tie = Percent::from_total(tie, n);
    let green = Percent::from_total(green, n);
    println!("stats red:{:} tie:{:} green:{:}%", red, tie, green);
}

struct Percent(f32);

impl Percent {
    pub fn from_total(n: usize, total: usize) -> Self {
        Self(n as f32 / total as f32)
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1$}%", 100.0 * self.0, f.precision().unwrap_or(1))
    }
}
