use regex::Regex;

use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_lines = read_input_to_string(3)?;

        let reg = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)|(?<modifier>don't\(\)|do\(\))")
            .unwrap();

        let (_, count) =
            reg.captures_iter(&input_lines)
                .fold((true, 0), |(should_process, count), c| {
                    if let Some(modifier) = c.name("modifier") {
                        return (modifier.as_str() == "do()", count);
                    }

                    if !should_process {
                        return (should_process, count);
                    }

                    let first: u32 = c["first"].parse().unwrap();
                    let second: u32 = c["second"].parse().unwrap();
                    let count = count + (first * second);

                    (should_process, count)
                });

        Ok(count)
    })
}
