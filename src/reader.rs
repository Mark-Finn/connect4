use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_positions(filepath: &str) -> Lines<BufReader<File>> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

pub fn line_to_position_score(line: String) -> (String, i8) {
    let split: Vec<&str> = line.split(' ').collect();
    match (split[0].to_string(), split[1].parse::<i8>()) {
        (s, Ok(i)) => (s, i),
        (s, ..) => panic!("Failed to parse test case {}", s),
    }
}