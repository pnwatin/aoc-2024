use regex::Regex;

use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_lines = read_input_to_string(3)?;
        let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

        let catpures = re.captures_iter(&input_lines);

        let count = catpures.fold(0, |count, c| {
            let first: i32 = c["first"].parse().unwrap();
            let second: i32 = c["second"].parse().unwrap();

            count + (first * second)
        });

        Ok(count)
    })
}
