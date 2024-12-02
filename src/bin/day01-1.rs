use aoc::{pretty_result, read_input};

fn main() {
    pretty_result(|| {
        let input_lines = read_input(1)?;

        let (mut left, mut right): (Vec<_>, Vec<_>) = input_lines
            .into_iter()
            .filter_map(|n| {
                n.split_once("   ")
                    .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
            })
            .unzip();

        left.sort_unstable();
        right.sort_unstable();

        let distance: usize = left
            .into_iter()
            .zip(right)
            .map(|(left, right)| left.abs_diff(right))
            .sum();

        Ok(distance)
    })
}
