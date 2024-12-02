use aoc::{pretty_result, read_input};

fn main() {
    pretty_result(|| {
        let input_lines = read_input(2)?;

        let lines = input_lines.into_iter().map(|n| {
            n.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        });

        let safe_count = lines.filter(|l| is_safe(l)).count();

        Ok(safe_count)
    })
}

fn is_safe(line: &[i32]) -> bool {
    let diff = line
        .windows(2)
        .map(|window| {
            let &[prev, curr] = window else {
                return 0;
            };

            curr - prev
        })
        .collect::<Vec<_>>();

    let is_ascending = diff.iter().all(|n| n > &0);
    let is_descending = diff.iter().all(|n| n < &0);
    let is_ordering_valid = is_descending || is_ascending;

    if !is_ordering_valid {
        return false;
    }

    diff.iter().fold(is_ascending || is_descending, |safe, n| {
        safe && n.abs() <= 3
    })
}
