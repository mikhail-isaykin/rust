use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_lines(path: &str) -> Result<usize, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}

fn main() {
    let file_path = "data.txt";

    match count_lines(file_path) {
        Ok(count) => println!("Total lines: {}", count),
        Err(error) => println!("Failed to read file: {}", error),
    }
}
