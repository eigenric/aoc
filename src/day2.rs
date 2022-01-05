use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Archivo no encontrado");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines
}

pub fn exercise2() {
    //let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let passwords = load_from_file("day2");

    let re = Regex::new(
            r"(\d+)-(\d+) (\w): (\w+)"
        ).unwrap();

    let mut nvalids = 0;
    for password in passwords {
        let cap = re.captures(&password).unwrap();
        let low: i32 = cap[1].parse::<i32>().unwrap();
        let high: i32 = cap[2].parse::<i32>().unwrap();
        let letter = &cap[3];
        let pass = String::from(&cap[4]);

        let ocurrences = pass.matches(letter).count() as i32;
        if low <= ocurrences && ocurrences <= high {
            nvalids += 1;
        }
    }
    println!("Hay {} passwords validas", nvalids);
}

pub fn part_two() {
    let passwords = load_from_file("day2");

    let re = Regex::new(
            r"(\d+)-(\d+) (\w): (\w+)"
        ).unwrap();

    let mut nvalids = 0;
    for password in passwords {
        let cap = re.captures(&password).unwrap();
        let low: i32 = cap[1].parse::<i32>().unwrap();
        let high: i32 = cap[2].parse::<i32>().unwrap();
        let letter = &cap[3];
        let pass = String::from(&cap[4]);
        
        let first_char = pass.chars().nth((low-1) as usize).unwrap().to_string();
        let second_char = pass.chars().nth((high-1) as usize).unwrap().to_string();

        let first_valid = first_char == letter && second_char != letter;
        let second_valid = first_char != letter && second_char == letter;

        if first_valid || second_valid {
            nvalids += 1;
        }
    }
    println!("Hay {} passwords validas", nvalids);
}