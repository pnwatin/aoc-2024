use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(4)?;
        let matrix: Vec<_> = input_str.lines().map(str::as_bytes).collect();

        let mut count = 0;

        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                let char = matrix[row][col];

                if char == b'X' {
                    count += count_xmas_at(&matrix, row, col);
                }
            }
        }

        Ok(count)
    })
}

fn count_xmas_at(matrix: &[&[u8]], row: usize, col: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|(dir_row, dir_col)| {
            (1..4).all(|i| {
                let row = row as isize + *dir_row * i as isize;
                let col = col as isize + *dir_col * i as isize;

                if row < 0 || col < 0 {
                    return false;
                }

                matrix.get(row as usize).and_then(|r| r.get(col as usize)) == Some(&b"XMAS"[i])
            })
        })
        .count()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
