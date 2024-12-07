use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(6)?;
        let mut matrix: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();

        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut current_direction = 0;

        let mut guard = guard_position(&matrix);

        while let Some((y, x)) = guard {
            let (dy, dx) = directions[current_direction];
            let next_y = (y + dy) as usize;
            let next_x = (x + dx) as usize;

            let next_char = &matrix
                .get(next_y)
                .and_then(|l| l.get(next_x))
                .unwrap_or(&'s');

            match next_char {
                's' => {
                    matrix[y as usize][x as usize] = 'X';
                    guard = None;
                }
                '#' => {
                    current_direction = (current_direction + 1) % 4;
                }
                _ => {
                    matrix[next_y][next_x] = '^';
                    matrix[y as usize][x as usize] = 'X';
                    guard = Some((next_y as isize, next_x as isize));
                }
            }
        }

        let positions: usize = matrix
            .into_iter()
            .map(|l| l.into_iter().filter(|c| *c == 'X').count())
            .sum();

        Ok(positions)
    })
}

fn guard_position(matrix: &[Vec<char>]) -> Option<(isize, isize)> {
    let mut positon = None;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == '^' {
                positon = Some((y as isize, x as isize));
                break;
            }
        }
    }

    positon
}
