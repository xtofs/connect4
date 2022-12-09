#![feature(associated_type_bounds)]
mod bitboards;
mod grid_patterns;
mod percentage;

pub use percentage::*;

use bitboards::BitBoard;
use grid_patterns::find_four_in_a_row;
use std::fmt::Display;
use std::ops::{Index, Not};

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

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Board {
    pub boards: [BitBoard; 2],
    pub turn: Player,
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

impl Move {
    pub fn new(player: Player, column: usize) -> Move {
        Self { column, player }
    }
}

impl Board {
    const W: usize = 7;
    const H: usize = 6;

    /// row by row indices
    fn indices() -> impl Iterator<Item = [usize; 2]> {
        (0..Board::H).flat_map(|h| (0..Board::W).map(move |w| [h, w]))
    }

    pub fn moves(&self) -> Vec<Move> {
        let player = self.turn;
        (0..7usize)
            .filter(|&column| self.height(column) < 6)
            .map(|column| Move::new(player, column))
            .collect()
    }

    fn drop_into(&mut self, column: usize, player: Player) {
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

    pub fn play(&self, mv: &Move) -> Board {
        let mut board = self.clone();
        board.drop_into(mv.column, mv.player);
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
}

const RED: &'static str = "\x1b[31m";
const CLR: &'static str = "\x1b[39m";

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid: [[char; Board::W]; Board::H] = self.into();

        if f.alternate() {
            let highlights = find_four_in_a_row(&grid).unwrap();

            for i in (0..Board::H).rev() {
                for j in 0..Board::W {
                    let ch = grid[i][j];
                    if highlights.get(&[i, j]).is_some() {
                        write!(f, "{RED}{ch}{CLR} ")?;
                    } else {
                        write!(f, "{ch} ")?;
                    };
                }
                writeln!(f)?;
            }
        } else {
            for i in (0..Board::H).rev() {
                for j in 0..Board::W {
                    let ch = grid[i][j];
                    write!(f, "{ch} ")?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<[usize; 2]> for Board {
    type Output = Option<Player>;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        if self.boards[Player::X as usize].get(7 - index[0], index[1]) {
            &Some(Player::X)
        } else if self.boards[Player::O as usize].get(7 - index[0], index[1]) {
            &Some(Player::O)
        } else {
            &None
        }
    }
}

/// convert board to character grid
impl From<&Board> for [[char; Board::W]; Board::H] {
    fn from(board: &Board) -> Self {
        let mut grid = Self::default();
        for ix in Board::indices() {
            grid[ix[0]][ix[1]] = match board[ix] {
                Some(Player::X) => 'X',
                Some(Player::O) => 'O',
                None => '.',
            }
        }
        grid
    }
}
