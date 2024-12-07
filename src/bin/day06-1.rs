use std::collections::HashSet;

use aoc::{pretty_result, read_input_to_string};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(6)?;

        let matrix: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();
        let height = matrix.len();
        let width = matrix[0].len();

        let (mut y, mut x) = get_guard(&matrix).unwrap();
        let mut dir_idx = 0;
        let mut visited = HashSet::new();

        loop {
            let (dy, dx) = DIRECTIONS[dir_idx];
            let ny = y.wrapping_add(dy as usize);
            let nx = x.wrapping_add(dx as usize);

            if ny >= height || nx >= width {
                break;
            }

            let next_char = matrix[ny][nx];

            match next_char {
                '#' => {
                    dir_idx = (dir_idx + 1) % 4;
                }
                _ => {
                    let npid = ny * width + nx;
                    visited.insert(npid);

                    y = ny;
                    x = nx;
                }
            }
        }

        Ok(visited.len())
    })
}

fn get_guard(matrix: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, row) in matrix.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            return Some((y, x));
        }
    }
    None
}
