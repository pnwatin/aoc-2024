use std::fmt::Display;

mod day01;

pub use day01::Day01;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub trait AdventDay {
    fn day_number(&self) -> u32;
    fn part_one(&self) -> Result<Box<dyn Display>>;
    fn part_two(&self) -> Result<Box<dyn Display>>;
}
