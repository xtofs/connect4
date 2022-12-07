#![allow(non_upper_case_globals)]

use crate::{Board, Player};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct Pattern<const N: usize> {
    cells: [[usize; 2]; N],
    w: usize,
    h: usize,
}

lazy_static! {
    pub(crate) static ref diagonal: Pattern<4> = Pattern::new([[0, 0], [1, 1], [2, 2], [3, 3]]);
    pub(crate) static ref antidiagonal: Pattern<4> = Pattern::new([[0, 3], [1, 2], [2, 1], [3, 0]]);
    pub(crate) static ref vertical: Pattern<4> = Pattern::new([[0, 0], [0, 1], [0, 2], [0, 3]]);
    pub(crate) static ref horizontal: Pattern<4> = Pattern::new([[0, 0], [1, 0], [2, 0], [3, 0]]);
}

lazy_static! {
    pub(crate) static ref pattens: [&'static Pattern<4>; 4] = [
        horizontal.deref(),
        vertical.deref(),
        diagonal.deref(),
        antidiagonal.deref(),
    ];
}

impl<const N: usize> Pattern<N> {
    fn new(cells: [[usize; 2]; N]) -> Self {
        let w = cells.into_iter().map(|[i, _]| i).max().unwrap();
        let h = cells.into_iter().map(|[_, j]| j).max().unwrap();
        Self { cells, w, h }
    }

    pub fn matches<P: PartialEq + Copy, const W: usize, const H: usize, G: Grid<P, W, H>>(
        &self,
        grid: &G,
        player: P,
    ) -> Option<HashSet<[usize; 2]>> {
        for i in 0..W - self.w {
            for j in 0..H - self.h {
                let check = self.cells.map(|[k, l]| [i + k, j + l]);
                if check
                    .into_iter()
                    .all(move |[a, b]| grid.get([a, b]) == Some(player))
                {
                    return Some(check.into_iter().collect());
                }
            }
        }
        None
    }
}

pub(crate) trait Grid<P, const W: usize, const H: usize> {
    fn get(&self, index: [usize; 2]) -> Option<P>;
}

impl<const W: usize, const H: usize, P: PartialEq + Copy> Grid<P, W, H> for [[Option<P>; W]; H] {
    fn get(&self, index: [usize; 2]) -> Option<P> {
        self[index[0]][index[1]]
    }
}

#[derive(Debug, Default)]
pub(crate) struct Connect4Grid([[Option<Player>; 7]; 6]);

impl Connect4Grid {
    fn four(&self) -> HashSet<[usize; 2]> {
        for pattern in pattens.into_iter() {
            for player in [Player::X, Player::O] {
                if let Some(set) = pattern.matches(self, player) {
                    return set;
                }
            }
        }
        HashSet::default()
    }
}

impl Grid<Player, 7, 6> for Connect4Grid {
    fn get(&self, index: [usize; 2]) -> Option<Player> {
        self.0[index[1]][index[0]]
    }
}

impl From<&Board> for Connect4Grid {
    fn from(board: &Board) -> Self {
        let mut c4 = Connect4Grid::default();

        for i in (0..6).rev() {
            for j in 0..7 {
                c4.0[i][j] = board.cell(i, j);
            }
        }

        c4
    }
}

impl Display for Connect4Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hl = self.four();
        for r in 0..self.0.len() {
            let row = self.0[r];
            for c in 0..row.len() {
                let ch = match self.0[r][c] {
                    Some(Player::X) => 'X',
                    Some(Player::O) => 'O',
                    None => '_',
                };
                if hl.get(&[c, r]).is_some() {
                    write!(f, "\x1b[31m{ch}\x1b[39m")?;
                } else {
                    write!(f, "{ch}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
