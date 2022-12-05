pub mod percentage;
pub use percentage::*;

use std::{
    fmt::{Binary, Display},
    ops::Not,
};

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Player {
    #[default]
    X,
    O,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Move {
    color: Player,
    column: usize,
}

impl Move {
    fn new(color: Player, column: usize) -> Move {
        Self { column, color }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct BitBoard(u64);

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Board {
    pub turn: Player,
    boards: [BitBoard; 2],
}

impl Board {
    pub fn height(&self, column: usize) -> usize {
        self.boards
            .iter()
            .map(|bb| bb.height(column))
            .max()
            .unwrap()
    }

    pub fn moves(&self) -> Vec<Move> {
        let color = self.turn;
        (0..7usize)
            .filter(|&column| self.height(column) < 6)
            .map(|column| Move::new(color, column))
            .collect()
    }

    fn drop(&mut self, column: usize, color: Player) {
        let h = self
            .boards
            .iter()
            .map(|bb| bb.height(column))
            .max()
            .unwrap();
        let bb = &mut self.boards[color as usize];
        *bb = bb.set(h, column, true)
    }

    pub fn play(&mut self, mv: &Move) -> Board {
        let mut board = self.clone();
        board.drop(mv.column, mv.color);
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
        if (0..7).all(|c| self.boards.iter().map(|bb| bb.height(c)).max().unwrap() >= 6) {
            return Some(None);
        }

        None
    }

    fn cell(&self, i: usize, j: usize) -> Option<Player> {
        if self.boards[Player::X as usize].get(i, j) {
            Some(Player::X)
        } else if self.boards[Player::O as usize].get(i, j) {
            Some(Player::O)
        } else {
            None
        }
    }
}

impl BitBoard {
    pub fn get(&self, row: usize, col: usize) -> bool {
        self.column(col) & 1 << row != 0
    }

    #[inline]
    fn column(&self, col: usize) -> u8 {
        let bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        bytes[col]
    }

    pub fn height(&self, col: usize) -> usize {
        8 - self.column(col).leading_zeros() as usize
    }

    pub fn set(&self, row: usize, col: usize, val: bool) -> BitBoard {
        let mut bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        if val {
            bytes[col] |= 1 << row;
        } else {
            bytes[col] &= !(1 << row);
        }
        BitBoard(unsafe { std::intrinsics::transmute(bytes) })
    }

    pub fn has_four(&self) -> bool {
        let v = self.0;

        ((v << 0) & (v << 8) & (v << 16) & (v << 24)) != 0
            || ((v << 0) & (v << 1) & (v << 2) & (v << 3)) != 0
            || ((v << 0) & (v << 7) & (v << 14) & (v << 21)) != 0
            || ((v << 0) & (v << 9) & (v << 18) & (v << 27)) != 0
    }
}

impl Binary for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        for byte in bytes {
            write!(f, "{:08b}", byte)?;
            write!(f, " ")?;
        }
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..6).rev() {
            for j in 0..7 {
                let ch = match self.cell(i, j) {
                    Some(Player::X) => 'X',
                    Some(Player::O) => 'O',
                    None => '.',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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

// .X.....
// .O.....
// .X.O...
// .XOO.OX
// .OXOOXX
// XOXOXOX
