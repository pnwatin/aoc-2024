use std::collections::HashSet;

use aoc::{pretty_result, read_input_to_string};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(6)?;

        let matrix: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();
        let height = matrix.len();
        let width = matrix[0].len();

        let guard = get_guard(&matrix).unwrap();
        let mut count = 0;
        let mut visited = HashSet::new();

        for (start_y, row) in matrix.iter().enumerate() {
            for (start_x, &cell) in row.iter().enumerate() {
                if cell == '#' || cell == '^' {
                    continue;
                }

                visited.clear();

                let (mut y, mut x) = guard;
                let mut dir_idx = 0;

                loop {
                    let (dy, dx) = DIRECTIONS[dir_idx];
                    let ny = y.wrapping_add(dy as usize);
                    let nx = x.wrapping_add(dx as usize);

                    if ny >= height || nx >= width {
                        break;
                    }

                    let next_char = if ny == start_y && nx == start_x {
                        '#'
                    } else {
                        matrix[ny][nx]
                    };

                    match next_char {
                        '#' => {
                            let pid = y * width + x;
                            let npid = ny * width + nx;
                            let id = ((pid as u64) << 32) | npid as u64;

                            if !visited.insert(id) {
                                count += 1;
                                break;
                            }

                            dir_idx = (dir_idx + 1) % 4;
                        }
                        _ => {
                            y = ny;
                            x = nx;
                        }
                    }
                }
            }
        }

        Ok(count)
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
