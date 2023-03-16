use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line.trim().to_string());
        }
    }

    return lines;
}
