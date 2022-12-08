use connect4::Board;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut board = Board::default();
    let mut rng = thread_rng();

    #[allow(unused_variables)]
    let mut half_moves = 0;

    loop {
        half_moves += 1;
        match board.state() {
            Some(state) => {
                println!("after {half_moves} moves: game ended {:?}", state);
                println!("{:#}", board);
                break;
            }
            _ => {}
        }

        let moves = board.moves();
        if let Some(mv) = moves.choose(&mut rng) {
            half_moves += 1;
            board = board.play(&mv);
        } else {
            // no more moves
            println!(
                "after {half_moves} moves: no more moves for {:?}",
                board.turn,
            );
            break;
        }
    }
}
