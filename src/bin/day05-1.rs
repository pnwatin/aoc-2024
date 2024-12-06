use std::{cmp::Ordering, collections::HashMap};

use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(5)?;
        let (rules, updates) = input_str.split_once("\n\n").unwrap();

        let order: HashMap<(usize, usize), Ordering> = rules
            .lines()
            .filter_map(|line| {
                line.split_once('|').map(|(f, s)| {
                    (
                        f.trim().parse::<usize>().unwrap(),
                        s.trim().parse::<usize>().unwrap(),
                    )
                })
            })
            .map(|(before, after)| ((before, after), Ordering::Less))
            .collect();

        let valid_updates_sum: usize = updates
            .lines()
            .filter_map(|l| {
                let sequence = l
                    .split(',')
                    .map(|sn| sn.parse().unwrap())
                    .collect::<Vec<usize>>();

                sequence
                    .is_sorted_by(|&a, &b| order.get(&(a, b)) == Some(&Ordering::Less))
                    .then(|| sequence[sequence.len() / 2])
            })
            .sum();

        Ok(valid_updates_sum)
    })
}
