use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn loadints_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("Archivo no encontrado");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    numbers
}

pub fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Archivo no encontrado");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines
}