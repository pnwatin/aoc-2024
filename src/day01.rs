use crate::{AdventDay, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "./inputs/day01.txt";

pub struct Day01;

impl AdventDay for Day01 {
    fn day_number(&self) -> u32 {
        1
    }

    fn part_one(&self) -> Result<Box<dyn std::fmt::Display>> {
        let (mut left, mut right) = read_and_format_input(INPUT_PATH)?;

        left.sort_unstable();
        right.sort_unstable();

        let distance: u32 = left
            .into_iter()
            .zip(right)
            .map(|(left, right)| left.abs_diff(right))
            .sum();

        Ok(Box::new(distance))
    }

    fn part_two(&self) -> Result<Box<dyn std::fmt::Display>> {
        let (left, right) = read_and_format_input(INPUT_PATH)?;

        let frequency_map = right.into_iter().fold(HashMap::new(), |mut map, loc| {
            *map.entry(loc).or_insert(0) += 1;
            map
        });

        let similarity: u32 = left
            .into_iter()
            .map(|l| frequency_map.get(&l).unwrap_or(&0) * l)
            .sum();

        Ok(Box::new(similarity))
    }
}

fn read_and_format_input(file_path: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open(file_path)?;
    let buf = BufReader::new(file);

    let mut left_locations = Vec::new();
    let mut right_locations = Vec::new();

    for line_result in buf.lines() {
        let line = line_result?;

        let mut locations = line.split_whitespace().map(|s| s.parse::<u32>());

        let left = locations
            .next()
            .ok_or_else(|| format!("Line `{}` is missing the first number", line))??;

        let right = locations
            .next()
            .ok_or_else(|| format!("Line `{}` is missing the second number", line))??;

        left_locations.push(left);
        right_locations.push(right);
    }

    Ok((left_locations, right_locations))
}
