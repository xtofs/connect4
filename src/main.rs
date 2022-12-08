use connect4::{Board, Move, Player};

fn main() {
    let mut b = Board::default();

    b = b.play(&Move::new(Player::O, 2));
    b = b.play(&Move::new(Player::X, 2));
    b = b.play(&Move::new(Player::X, 3));
    b = b.play(&Move::new(Player::O, 1));
    b = b.play(&Move::new(Player::O, 1));
    b = b.play(&Move::new(Player::X, 1));

    b = b.play(&Move::new(Player::O, 0));
    b = b.play(&Move::new(Player::O, 0));
    b = b.play(&Move::new(Player::O, 0));
    b = b.play(&Move::new(Player::X, 0));

    println!("{:#}", b);
}
