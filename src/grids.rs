use std::{collections::HashSet, fmt::Display, ops::Index, str::FromStr};

#[derive(Debug, Default)]
pub struct Grid([[char; 7]; 6]);

impl Grid {
    const PATTERNS: [([usize; 2], [[usize; 2]; 4]); 4] = [
        ([0, 3], [[0, 0], [0, 1], [0, 2], [0, 3]]),
        ([3, 0], [[0, 0], [1, 0], [2, 0], [3, 0]]),
        ([3, 3], [[0, 0], [1, 1], [2, 2], [3, 3]]),
        ([3, 3], [[3, 0], [2, 1], [1, 2], [0, 3]]),
    ];

    /// an Iterator of all indices of rows of four in all directions over the grid
    pub fn indices_of_four_connected(&self) -> impl Iterator<Item = [[usize; 2]; 4]> + '_ {
        std::iter::from_generator(|| {
            for pat in Self::PATTERNS.into_iter() {
                for i in 0..(6 - pat.0[0]) {
                    for j in 0..(7 - pat.0[1]) {
                        let ix = pat.1.map(|off| [i + off[0], j + off[1]]);
                        yield ix
                    }
                }
            }
        })
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [['\0'; 7]; 6];
        let mut row = 0;
        let mut col = 0;
        for ch in s.chars() {
            match ch {
                'X' | 'x' => {
                    grid[row][col] = 'X';
                    col += 1;
                }
                'O' | 'o' => {
                    grid[row][col] = 'O';
                    col += 1;
                }
                ' ' | '.' | '_' => {
                    grid[row][col] = '.';
                    col += 1;
                }
                '|' | '\n' => {
                    row += 1;
                    col = 0;
                }
                _ => return Err(format!("unknown character {ch}")),
            }
        }

        Ok(Grid(grid))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..(6) {
            for j in 0..(7) {
                write!(f, "{} ", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<[usize; 2]> for Grid {
    type Output = char;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;
        &self.0[i][j]
    }
}

// ####################

#[derive(Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

#[derive(Debug)]
pub struct HighlightedGrid<'a> {
    grid: &'a Grid,
    color: Color,
    highlights: HashSet<[usize; 2]>,
}

impl Color {
    fn write_foreground_code(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[")?;
        match *self {
            Color::Black => write!(f, "30"),
            Color::Red => write!(f, "31"),
            Color::Green => write!(f, "32"),
            Color::Yellow => write!(f, "33"),
            Color::Blue => write!(f, "34"),
            Color::Purple => write!(f, "35"),
            Color::Cyan => write!(f, "36"),
            Color::White => write!(f, "37"),
        }?;
        write!(f, "m")
    }
}

impl<'a> HighlightedGrid<'a> {
    pub fn new<I>(grid: &'a Grid, highlights: I, color: Color) -> Self
    where
        I: Into<HashSet<[usize; 2]>>,
    {
        HighlightedGrid {
            grid,
            color,
            highlights: highlights.into(),
        }
    }
}

impl<'a> Display for HighlightedGrid<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..6 {
            for j in 0..7 {
                let ix = [i, j];
                let ch = self.grid.0[i][j];
                if self.highlights.get(&ix).is_some() {
                    self.color.write_foreground_code(f)?;
                    write!(f, "{} ", ch)?;
                    write!(f, "\x1B[0m")?;
                } else {
                    write!(f, "{} ", ch)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn number_of_row_of_fours() {
        let grid = Grid::default();

        let count = grid.indices_of_four_connected().map(|_| 1u32).sum::<u32>();
        // 4*6 + 3*7 + 4*3 + 4*3 == 69
        // h     v     d     a
        assert_eq!(69, count);
    }
}
