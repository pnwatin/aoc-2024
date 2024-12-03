use std::{
    fs::{self, File},
    io::BufReader,
};

pub fn read_input(day_number: usize) -> std::io::Result<BufReader<File>> {
    let input_path = format!("./inputs/day{:02}.txt", day_number);
    let file = File::open(input_path)?;

    Ok(BufReader::new(file))
}

pub fn read_input_to_string(day_number: usize) -> std::io::Result<String> {
    let input_path = format!("./inputs/day{:02}.txt", day_number);

    fs::read_to_string(input_path)
}

pub fn pretty_result<F, S>(f: F)
where
    S: std::fmt::Display,
    F: Fn() -> std::io::Result<S>,
{
    let start = std::time::Instant::now();

    match f() {
        Ok(result) => {
            println!("  solution : {}", result);
        }
        Err(e) => {
            println!("  error : {}", e);
        }
    };

    println!("  took : {:?}", start.elapsed());
}
