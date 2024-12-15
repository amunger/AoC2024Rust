use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_input() -> Vec<String> {
    let mut input = vec![];
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}