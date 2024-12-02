pub fn read_input(day_number: usize) -> std::io::Result<Vec<String>> {
    use std::io::BufRead;

    let input_path = format!("./inputs/day{:02}.txt", day_number);
    let file = std::fs::File::open(input_path)?;
    let bufreader = std::io::BufReader::new(file);

    bufreader.lines().collect()
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
