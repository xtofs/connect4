use std::{
    fmt::{Binary, Display},
    ops::Not,
};

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    #[default]
    Red,
    Green,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Move {
    color: Color,
    column: usize,
}

impl Move {
    fn new(color: Color, column: usize) -> Move {
        Self { column, color }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct BitBoard(u64);

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Board {
    pub turn: Color,
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

    pub fn drop(&mut self, column: usize, color: Color) {
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
    pub fn state(&self) -> Option<Option<Color>> {
        if self.boards[Color::Green as usize].has_four() {
            return Some(Some(Color::Green));
        }
        if self.boards[Color::Red as usize].has_four() {
            return Some(Some(Color::Red));
        }
        // full?
        if (0..7).all(|c| self.boards.iter().map(|bb| bb.height(c)).max().unwrap() >= 6) {
            return Some(None);
        }

        None
    }

    fn cell(&self, i: usize, j: usize) -> Option<Color> {
        if self.boards[Color::Red as usize].get(i, j) {
            Some(Color::Red)
        } else if self.boards[Color::Green as usize].get(i, j) {
            Some(Color::Green)
        } else {
            None
        }
    }
}

// impl Not for Color {
//     type Output = Color;

//     fn not(self) -> Self::Output {
//         match self {
//             Color::Red => Color::Green,
//             Color::Green => Color::Red,
//         }
//     }
// }

// impl Display for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for i in 0..6 {
//             for j in 0..7 {
//                 match self.columns[j][i] {
//                     Some(Color::Red) => write!(f, "X")?,
//                     Some(Color::Green) => write!(f, "O")?,
//                     None => write!(f, " ")?,
//                 };
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }

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
impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        for i in 0..8 {
            for j in 0..8 {
                let ch = if (bytes[j] & (1 << i)) != 0 { 'X' } else { '.' };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..6).rev() {
            for j in 0..7 {
                let ch = match self.cell(i, j) {
                    Some(Color::Green) => 'X',
                    Some(Color::Red) => 'O',
                    None => '.',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Red => Color::Green,
            Color::Green => Color::Red,
        }
    }
}
