use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use itertools::Itertools;

fn load_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("Archivo no encontrado");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    numbers
}

pub fn exercise1() {

    let expenses = load_from_file("day1");
    let found = expenses.into_iter()
            .combinations(2)
            .filter(|v| v[0] + v[1] == 2020)
            .exactly_one().unwrap();
    let product = found[0] * found[1];
    println!("{} * {} = {}", found[0], found[1], product); 
}

pub fn part_two() {
    
    let expenses = load_from_file("day1");
    let found = expenses.into_iter()
            .combinations(3)
            .filter(|v| v[0] + v[1] + v[2] == 2020)
            .exactly_one().unwrap();
    let product = found[0] * found[1] * found[2];
    println!("{} * {} * {} = {}", found[0], found[1], found[2], product);
}
