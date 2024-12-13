use aoc::{pretty_result, read_input_to_string};

fn main() {
    pretty_result(|| {
        let input_str = read_input_to_string(9)?.trim().to_string();
        let bytes = input_str.as_bytes();
        let total_len = bytes.iter().map(|b| (b - b'0') as usize).sum();

        let mut parsed: Vec<i64> = Vec::with_capacity(total_len);

        let mut curr_index = 0;

        for (i, b) in bytes.iter().enumerate() {
            let len = (b - b'0') as usize;

            if len == 0 {
                continue;
            }

            if i % 2 == 0 {
                parsed.extend(std::iter::repeat(curr_index).take(len));
                curr_index += 1;
            } else {
                parsed.extend(std::iter::repeat(-1).take(len));
            }
        }

        let mut left = 0;
        let mut right = parsed.len() - 1;

        while left < right {
            while parsed[left] >= 0 {
                left += 1;
            }

            while parsed[right] < 0 {
                right -= 1;
            }

            if left < right {
                parsed[left] = parsed[right];
                parsed[right] = -1;
                left += 1;
                right -= 1;
            }
        }

        let checksum: usize = parsed
            .iter()
            .enumerate()
            .filter_map(|(i, &id)| {
                if id.is_negative() {
                    return None;
                }

                Some(i * id as usize)
            })
            .sum();

        Ok(checksum)
    })
}
