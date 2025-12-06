use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn read_file_line_by_line(path: PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");

    let reader = io::BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
}
