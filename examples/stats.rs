use connect4::{Board, Percent, Player};
use rand::{seq::SliceRandom, thread_rng};
use std::{env::args, time::SystemTime};

fn main() {
    let mut rng = thread_rng();
    let (mut x_count, mut tie_count, mut o_count) = (0, 0, 0);

    let n: usize = args().nth(1).map(|n| n.parse().unwrap()).unwrap_or(1000);

    println!("running: {} games", n);
    let start = SystemTime::now();
    for _ in 0..n {
        let mut board = Board::default();

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

            let moves = board.moves();
            if let Some(mv) = moves.choose(&mut rng) {
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
