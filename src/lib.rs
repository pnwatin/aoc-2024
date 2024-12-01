use std::{fmt::Display, time::Instant};

mod day01;

pub use day01::Day01;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub trait AdventDay {
    fn day_number(&self) -> u32;

    fn part_one(&self) -> Result<Box<dyn Display>>;

    fn part_two(&self) -> Result<Box<dyn Display>>;

    fn pretty(&self) {
        println!("\nDay {}", self.day_number());

        let part_one_start = Instant::now();
        let part_one_result = match self.part_one() {
            Ok(res) => res.to_string(),
            Err(e) => format!("Error: {}", e),
        };
        println!(
            "- ⭐    result : {} (took {:?})",
            part_one_result,
            part_one_start.elapsed(),
        );

        let part_two_start = Instant::now();
        let part_two_result = match self.part_two() {
            Ok(res) => res.to_string(),
            Err(e) => format!("Error: {}", e),
        };
        println!(
            "- ⭐⭐  result : {} (took {:?})",
            part_two_result,
            part_two_start.elapsed(),
        );
    }
}
