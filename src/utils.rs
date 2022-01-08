use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Archivo no encontrado");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines
}