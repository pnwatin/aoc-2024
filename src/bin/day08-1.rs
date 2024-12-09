use std::collections::{HashMap, HashSet};

use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(8)?;

        let matrix: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();

        let max_row = matrix.len() - 1;
        let max_col = matrix[0].len() - 1;

        let mut antinodes = HashSet::new();

        for frequency in get_frequencies(&matrix) {
            for (a, b) in frequency
                .iter()
                .flat_map(|&a| frequency.iter().map(move |&b| (a, b)))
            {
                if a == b {
                    continue;
                }

                if let Some(an) = get_antinode(a, b, (max_row, max_col)) {
                    antinodes.insert(an);
                }
            }
        }

        Ok(antinodes.len())
    })
}

fn get_antinode(
    a: (usize, usize),
    b: (usize, usize),
    (max_y, max_x): (usize, usize),
) -> Option<(usize, usize)> {
    let (ay, ax) = a;
    let (by, bx) = b;

    let y = ay as isize + (ay as isize - by as isize);
    let x = ax as isize + (ax as isize - bx as isize);

    if y < 0 || x < 0 || y as usize > max_y || x as usize > max_x {
        return None;
    }

    Some((y as usize, x as usize))
}

fn get_frequencies(matrix: &[Vec<char>]) -> impl Iterator<Item = Vec<(usize, usize)>> {
    let mut ret: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char != '.' {
                ret.entry(char).or_default().push((y, x));
            }
        }
    }

    ret.into_values().filter(|coords| coords.len() > 1)
}
