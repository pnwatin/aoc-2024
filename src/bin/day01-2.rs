use std::collections::HashMap;

use aoc::{pretty_result, read_input};

fn main() {
    pretty_result(|| {
        let input_lines = read_input(1)?;

        let (left, right): (Vec<_>, Vec<_>) = input_lines
            .into_iter()
            .filter_map(|n| {
                n.split_once("   ")
                    .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
            })
            .unzip();

        let frequency_map = right.into_iter().fold(HashMap::new(), |mut map, n| {
            *map.entry(n).or_insert(0) += 1;
            map
        });

        let similarity: usize = left
            .into_iter()
            .map(|n| frequency_map.get(&n).unwrap_or(&0) * n)
            .sum();

        Ok(similarity)
    })
}
