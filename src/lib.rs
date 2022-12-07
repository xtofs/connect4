#![feature(associated_type_bounds)]
// #![feature(const_generics)]
// #![feature(generic_const_exprs)]

mod bitboards;
mod c4grid;
mod grids;
pub mod percentage;

use bitboards::BitBoard;
use c4grid::Connect4Grid;
use grids::Grid;
pub use percentage::*;

use std::{
    fmt::Display,
    ops::{Index, Not},
};

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Player {
    #[default]
    X,
    O,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Move {
    player: Player,
    column: usize,
}

impl Move {
    fn new(player: Player, column: usize) -> Move {
        Self { column, player }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Board {
    pub turn: Player,
    boards: [BitBoard; 2],
}

impl Not for Player {
    type Output = Player;

    fn not(self) -> Self::Output {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Board {
    pub fn moves(&self) -> Vec<Move> {
        let player = self.turn;
        (0..7usize)
            .filter(|&column| self.height(column) < 6)
            .map(|column| Move::new(player, column))
            .collect()
    }

    fn drop(&mut self, column: usize, player: Player) {
        let h = self.height(column);
        let bb = &mut self.boards[player as usize];
        bb.set(h, column, true)
    }

    fn height(&self, column: usize) -> usize {
        self.boards
            .iter()
            .map(|bb| bb.height(column))
            .max()
            .unwrap() // safe because there is always two boards
    }

    pub fn play(&mut self, mv: &Move) -> Board {
        let mut board = self.clone();
        board.drop(mv.column, mv.player);
        board.turn = !board.turn;
        board
    }

    /// returns the final game state if there is one
    pub fn state(&self) -> Option<Option<Player>> {
        if self.boards[Player::O as usize].has_four() {
            return Some(Some(Player::O));
        }
        if self.boards[Player::X as usize].has_four() {
            return Some(Some(Player::X));
        }
        // full?
        if (0..7).all(|c| self.height(c) >= 6) {
            return Some(None);
        }

        None
    }

    // #[deprecated(since = "0.0.1")]
    // fn cell(&self, i: usize, j: usize) -> Option<Player> {
    //     if self.boards[Player::X as usize].get(i, j) {
    //         Some(Player::X)
    //     } else if self.boards[Player::O as usize].get(i, j) {
    //         Some(Player::O)
    //     } else {
    //         None
    //     }
    // }

    fn cell_symbol(&self, i: usize, j: usize) -> char {
        match self[[i, j]] {
            Some(Player::X) => 'X',
            Some(Player::O) => 'O',
            None => '.',
        }
    }

    // fn four_in_a_row(&self, player: Player) -> HashSet<[usize; 2]> {
    //     for i in 0..(6 - 4) {
    //         for j in 0..7 {
    //             let row = four_in_a_row(i, j, 1, 0);
    //             if row
    //                 .clone()
    //                 .into_iter()
    //                 .all(|[k, l]| self.cell(k, l) == Some(player))
    //             {
    //                 return row.into_iter().collect();
    //             }
    //         }
    //     }
    //     for i in 0..6 {
    //         for j in 0..(7 - 4) {
    //             let row = four_in_a_row(i, j, 0, 1);
    //             if row
    //                 .clone()
    //                 .into_iter()
    //                 .all(|[k, l]| self.cell(k, l) == Some(player))
    //             {
    //                 return row.into_iter().collect();
    //             }
    //         }
    //     }
    //     HashSet::default()
    // }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let c4: Connect4Grid = self.into();
            c4.fmt(f)
        } else {
            for i in (0..6).rev() {
                for j in 0..7 {
                    let ch = self.cell_symbol(i, j);
                    write!(f, "{}", ch)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}

impl Index<[usize; 2]> for Board {
    type Output = Option<Player>;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        if self.boards[Player::X as usize].get(index[0], index[1]) {
            &Some(Player::X)
        } else if self.boards[Player::O as usize].get(index[0], index[1]) {
            &Some(Player::O)
        } else {
            &None
        }
    }
}

impl Grid<Player, 7, 6> for Board {}
