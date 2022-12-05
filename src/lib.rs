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
    pub fn moves(&self) -> Vec<Move> {
        let color = self.turn;
        (0..7usize)
            .filter(|&column| self.height(column) < 6)
            .map(|column| Move::new(color, column))
            .collect()
    }

    fn drop(&mut self, column: usize, color: Player) {
        let h = self.height(column);
        let bb = &mut self.boards[color as usize];
        bb.set(h, column, true)
    }

    fn height(&self, column: usize) -> usize {
        self.boards
            .iter()
            .map(|bb| bb.height(column))
            .max()
            .unwrap()
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
        if (0..7).all(|c| self.height(c) >= 6) {
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

    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        let mut bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        if val {
            bytes[col] |= 1 << row;
        } else {
            bytes[col] &= !(1 << row);
        }
        unsafe { self.0 = std::intrinsics::transmute(bytes) };
    }

    pub fn has_four(&self) -> bool {
        let bb = self.0;
        let v = (bb << 0) & (bb << 8) & (bb << 16) & (bb << 24);
        let h = (bb << 0) & (bb << 1) & (bb << 2) & (bb << 3);
        let d = (bb << 0) & (bb << 7) & (bb << 14) & (bb << 21);
        let a = (bb << 0) & (bb << 9) & (bb << 18) & (bb << 27);

        v != 0 || h != 0 || d != 0 || a != 0
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
