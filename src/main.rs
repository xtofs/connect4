mod grids;

use grids::Grid;
use std::{collections::HashSet, fmt::Display, str::FromStr};

fn main() {
    #[allow(non_upper_case_globals)]
    let grid: C4 = "___X___|__X____|_X_O___|X__OO__|___O_O_|___O__O"
        .parse()
        .unwrap();

    println!("{:#}", grid);
    println!("v {:?}", grids::vertical.matches(&grid, Player::O));
    println!("d {:?}", grids::diagonal.matches(&grid, Player::O));
    println!("a {:?}", grids::antidiagonal.matches(&grid, Player::X));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Player {
    X,
    O,
}

#[derive(Debug, Default)]
struct C4([[Option<Player>; 7]; 6]);
impl C4 {
    fn four(&self) -> HashSet<[usize; 2]> {
        for pattern in grids::pattens.into_iter() {
            for player in [Player::X, Player::O] {
                if let Some(set) = pattern.matches(self, player) {
                    return set;
                }
            }
        }
        HashSet::default()
    }
}

impl Grid<Player, 7, 6> for C4 {
    fn get(&self, index: [usize; 2]) -> Option<Player> {
        self.0[index[1]][index[0]]
    }
}

impl FromStr for C4 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: C4 = C4::default();
        let mut r = 0;
        let mut c = 0;
        for ch in s.chars() {
            match ch {
                '\n' | '|' => {
                    r += 1;
                    c = 0;
                }
                'X' => {
                    grid.0[r][c] = Some(Player::X);
                    c += 1
                }
                'O' => {
                    grid.0[r][c] = Some(Player::O);
                    c += 1
                }
                '_' | ' ' | '.' => c += 1,
                _ => return Err(format!("unknown character {ch}")),
            }
        }
        Ok(grid)
    }
}

impl Display for C4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hl = if f.alternate() {
            self.four()
        } else {
            HashSet::default()
        };
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
