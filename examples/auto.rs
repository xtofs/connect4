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
                println!("{}", board);
                println!("{:#}", board);
                println!("game ended: {:?}", state);
                break;
            }
            _ => {}
        }

        let moves = board.moves();
        if let Some(mv) = moves.choose(&mut rng) {
            half_moves += 1;
            // let player = board.turn;
            // println!("{half_moves}: {player:?} {mv:?} ");
            board = board.play(&mv);
        } else {
            // no more moves
            println!("no more moves for {:?}", board.turn,);
            break;
        }
    }
}
