use std::{error::Error, fs, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("[AoC 2024] - pnwatin");

    let mut days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .map(|p| p.ok().unwrap().path())
        .filter(|path| path.extension().unwrap() == "rs")
        .filter_map(|p| p.file_stem()?.to_str().map(str::to_string))
        .collect::<Vec<_>>();

    days.sort_unstable();

    for day in &days {
        let stars = day
            .split('-')
            .last()
            .unwrap_or("1")
            .parse::<usize>()
            .unwrap();

        let cmd = Command::new("cargo")
            .args(["run", "--release", "-q", "--bin", day])
            .output()?;
        let output = String::from_utf8(cmd.stdout)?;
        println!("# {} {} \n{}", day, "*".repeat(stars), output);
    }

    Ok(())
}
