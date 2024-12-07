use std::collections::HashSet;

use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(6)?;
        let matrix: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();

        let guard = get_guard(&matrix).unwrap();
        let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let height = matrix.len();
        let width = matrix[0].len();

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
                    let (dy, dx) = dirs[dir_idx];
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
                            // let edge_id = y * width + x;
                            // let next_edge_id = ny * width + nx;
                            // let edge = ((edge_id as u64) << 32) | next_edge_id as u64;

                            if !visited.insert(((y, x), (ny, nx))) {
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
