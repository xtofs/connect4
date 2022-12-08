use lazy_static::lazy_static;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Pattern<const N: usize> {
    cells: [[usize; 2]; N],
    w: usize,
    h: usize,
}

lazy_static! {
    pub(crate) static ref FOUR_IN_A_ROW_PATTERNS: [Pattern<4>; 4] = [
        Pattern::new([[0, 0], [1, 1], [2, 2], [3, 3]]), // diagonal
        Pattern::new([[0, 3], [1, 2], [2, 1], [3, 0]]), // anti diagonal
        Pattern::new([[0, 0], [0, 1], [0, 2], [0, 3]]), // horizontal
        Pattern::new([[0, 0], [1, 0], [2, 0], [3, 0]]), // vertical
    ];
}

impl<const N: usize> Pattern<N> {
    fn new(cells: [[usize; 2]; N]) -> Self {
        let w = cells.into_iter().map(|[i, _]| i).max().unwrap();
        let h = cells.into_iter().map(|[_, j]| j).max().unwrap();
        Self { cells, w, h }
    }

    pub fn matches<const W: usize, const H: usize, P>(
        &self,
        grid: &[[char; W]; H],
        predicate: P,
    ) -> Option<HashSet<[usize; 2]>>
    where
        P: Fn(char) -> bool,
    {
        for w in 0..W - self.w {
            for h in 0..H - self.h {
                let pat = self.cells.map(|[k, l]| [w + k, h + l]);
                if pat.into_iter().all(|ix| predicate(grid[ix[0]][ix[1]])) {
                    return Some(pat.into_iter().collect());
                }
            }
        }
        None
    }
}

pub fn find_four_in_a_row<const W: usize, const H: usize>(
    grid: &[[char; W]; H],
) -> Option<HashSet<[usize; 2]>> {
    for piece in ['X', 'O'] {
        for pattern in FOUR_IN_A_ROW_PATTERNS.iter() {
            if let Some(set) = pattern.matches(grid, |ch| ch == piece) {
                return Some(set);
            }
        }
    }
    None
}
