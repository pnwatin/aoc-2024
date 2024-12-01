use std::time::Instant;

use aoc::{AdventDay, Day01};

fn pretty_print_day(day: &dyn AdventDay) {
    println!("\nDay {}", day.day_number());

    let part_one_start = Instant::now();
    let part_one_result = match day.part_one() {
        Ok(res) => res.to_string(),
        Err(e) => format!("Error: {}", e),
    };
    println!(
        "- ⭐    result : {} (took {:?})",
        part_one_result,
        part_one_start.elapsed(),
    );

    let part_two_start = Instant::now();
    let part_two_result = match day.part_two() {
        Ok(res) => res.to_string(),
        Err(e) => format!("Error: {}", e),
    };
    println!(
        "- ⭐⭐  result : {} (took {:?})",
        part_two_result,
        part_two_start.elapsed(),
    );
}

fn main() {
    println!("AoC 2024 [pwnatin]");

    let days: Vec<&dyn AdventDay> = vec![&Day01];

    for day in days {
        pretty_print_day(day)
    }
}
