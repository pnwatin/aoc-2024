use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(4)?;
        let matrix: Vec<_> = input_str.lines().map(str::as_bytes).collect();

        let mut count = 0;

        if matrix.len() < 3 || matrix[0].len() < 3 {
            return Ok(count);
        }

        for row in 1..matrix.len() - 1 {
            for col in 1..matrix[0].len() - 1 {
                let char = matrix[row][col];

                if char == b'A' {
                    count += is_x_mas_at(&matrix, row, col) as usize;
                }
            }
        }

        Ok(count)
    })
}

fn is_x_mas_at(matrix: &[&[u8]], row: usize, col: usize) -> bool {
    let left_to_right = [matrix[row - 1][col - 1], matrix[row + 1][col + 1]];
    let right_to_left = [matrix[row + 1][col - 1], matrix[row - 1][col + 1]];

    [left_to_right, right_to_left]
        .iter()
        .all(|d| d == b"MS" || d == b"SM")
}
