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
            while parsed[right] < 0 {
                right -= 1;
            }

            let right_len = (0..)
                .take_while(|i| parsed[right - i] == parsed[right])
                .count();

            while left < right {
                while left < right && parsed[left] >= 0 {
                    left += 1;
                }

                let left_len = (0..)
                    .take_while(|i| parsed[left + i] == parsed[left])
                    .count();

                if right_len <= left_len && left < right {
                    for i in 0..right_len {
                        parsed[left + i] = parsed[right - i];
                        parsed[right - i] = -1;
                    }

                    break;
                }

                left += left_len;
            }

            left = 0;
            right = right.saturating_sub(right_len);
        }

        let mut checksum = 0;

        for (i, v) in parsed.into_iter().enumerate() {
            if v > 0 {
                checksum += i * v as usize;
            }
        }

        Ok(checksum)
    })
}
