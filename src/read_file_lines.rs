use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = match File::open(file_path) {
        Ok(ok_file) => ok_file,
        Err(err) => {
            eprintln!(
                "{}: {}\nMake sure to run as 'cargo run cargo run .\\src\\main.rs'",
                err, file_path
            );
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line.trim().to_string());
        }
    }

    return lines;
}
