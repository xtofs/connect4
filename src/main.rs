use connect4::{Color, Grid, HighlightedGrid};

fn main() {
    let grid: Grid = ".......|..XOOO.|..XXXX.|....X..|.......|......."
        .parse()
        .expect("not parsable");

    println!("{}", grid);

    for four in grid.indices_of_four_connected() {
        let chars = four.map(|ix| grid[ix]);
        if let Some(ch) = is_three_of_four_equal(&chars) {
            println!("candidate: {ch}  {four:?} ");
            println!("{}", HighlightedGrid::new(&grid, four, Color::Yellow));
        }
        if let Some(ch) = is_four_equal(&chars) {
            println!("win: {ch} {four:?} ");
            println!("{}", HighlightedGrid::new(&grid, four, Color::Green));
        }
    }
}

fn is_four_equal(four: &[char; 4]) -> Option<char> {
    match four {
        &[a, b, c, d] if a != '.' && a == b && b == c && c == d => Some(a),
        _ => None,
    }
}

fn is_three_of_four_equal(four: &[char; 4]) -> Option<char> {
    match four {
        &[a, b, c, '.'] if a != '.' && a == b && b == c => Some(a),
        &['.', a, b, c] if a != '.' && a == b && b == c => Some(a),
        _ => None,
    }
}
