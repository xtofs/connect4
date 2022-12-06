#![allow(non_upper_case_globals)]
use std::collections::HashSet;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Pattern<const N: usize> {
    cells: [[usize; 2]; N],
    w: usize,
    h: usize,
}

lazy_static! {
    pub static ref diagonal: Pattern<4> = Pattern::new([[0, 0], [1, 1], [2, 2], [3, 3]]);
    pub static ref antidiagonal: Pattern<4> = Pattern::new([[0, 3], [1, 2], [2, 1], [3, 0]]);
    pub static ref vertical: Pattern<4> = Pattern::new([[0, 0], [0, 1], [0, 2], [0, 3]]);
    pub static ref horizontal: Pattern<4> = Pattern::new([[0, 0], [1, 0], [2, 0], [3, 0]]);
}

lazy_static! {
    pub static ref pattens: [&'static Pattern<4>; 4] = [
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

pub trait Grid<P, const W: usize, const H: usize> {
    fn get(&self, index: [usize; 2]) -> Option<P>;
}

impl<const W: usize, const H: usize, P: PartialEq + Copy> Grid<P, W, H> for [[Option<P>; W]; H] {
    fn get(&self, index: [usize; 2]) -> Option<P> {
        self[index[0]][index[1]]
    }
}
