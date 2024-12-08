use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(7)?;

        let equations = input_str.lines().filter_map(|l| {
            l.split_once(':').map(|(value, remaining)| {
                (
                    value.parse::<u64>().unwrap(),
                    remaining
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>(),
                )
            })
        });

        let total: u64 = equations.filter(can_be_true).map(|(v, _)| v).sum();

        Ok(total)
    });
}

fn can_be_true((value, numbers): &(u64, Vec<u64>)) -> bool {
    for mut i in 0..3usize.pow((numbers.len() - 1) as u32) {
        let mut acc = numbers[0];

        for num in numbers.iter().skip(1) {
            match i % 3 {
                0 => acc += num,
                1 => acc *= num,
                2 => {
                    acc = acc * 10u64.pow(num.checked_ilog10().unwrap_or(0) + 1) + num;
                }
                _ => unreachable!(),
            }

            i /= 3;
        }

        if *value == acc {
            return true;
        }
    }

    false
}
