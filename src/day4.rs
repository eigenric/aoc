use regex::Regex;
use std::fs;
use std::collections::HashMap;

// Checks if two vectors contains the same elements (unordered)
pub fn is_equal(v1: &Vec<&str>, v2: &Vec<&str>) -> bool {
    let matching = v1.iter()
                     .zip(v2.iter())
                     .filter(|&(a,b)| a == b)
                     .count();
    //println!("Coincidencias: {}", matching);
    matching == v1.len() 
}

// Checks if a passport has valid keys and optionally valid values
pub fn is_valid(passport: &HashMap<&str, &str>, inner: bool) -> bool {
    let keys: Vec<&str> = vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];
    let mut passport_keys: Vec<_> = passport.keys().copied().collect();
    passport_keys.sort();

    let valid_keys = match passport_keys.len() {
        7 => !passport_keys.contains(&"cid"),
        8 => is_equal(&passport_keys, &keys),
        _ => false,
    };

    if inner {
        let valid_values = passport.iter()
                                .map(|(&key, value)|
                                    valid_keys && valid_value(key, value))
                                .all(|x| x);
        valid_keys && valid_values
    } else {
        valid_keys
    }
}

// Checks if a certain year is between two given.
pub fn year_between(value: &str, year1: u32, year2: u32) -> bool {
    let year: u32 = value.parse::<u32>().unwrap();
    year1 <= year && year <= year2
}

// Checks if eye colour is exactly one of the following.
pub fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

// Checks if hair colour is a proper #hex colour
pub fn validate_hcl(value: &str) -> bool {
    let re = Regex::new(r"#[\da-f]{6}").unwrap();
    re.is_match(value)
}

// Capture height and check if is in valid range
// If cm: [150,193]
// Else if in: [59, 76]
pub fn validate_hgt(value: &str) -> bool {
    let re = Regex::new(r"(\d+)(cm|in)").unwrap();

    if re.is_match(value) {
        let cap = re.captures(value).unwrap();
        let height: i32 = cap[1].parse::<i32>().unwrap();
        let unit = &cap[2];

        if unit == "cm" {
            150 <= height && height <= 193
        } else {
            59 <= height && height <= 76
        }
    } else {
        false
    }
}

// Checks is pid is a 9 digit number.
pub fn validate_pid(value: &str) -> bool {
    let svalue = String::from(value);
    svalue.chars().all(|c| c.is_numeric()) && svalue.len() == 9
}

// Checks each type of entry and returns true if valid
pub fn valid_value(key: &str, value: &str) -> bool{
    match key {
        "byr" => year_between(value, 1920, 2002),
        "cid" => true,
        "ecl" => validate_ecl(value),
        "eyr" => year_between(value, 2020, 2030),
        "hcl" => validate_hcl(value),
        "hgt" => validate_hgt(value),
        "iyr" => year_between(value, 2010, 2020),
        "pid" => validate_pid(value),
        _ => false,
    }
}

pub fn exercise1() {
    let input: String = fs::read_to_string("day4").expect("No se pudo leer");
    let raw_passports: Vec<&str> = input.split("\n\n").collect();

    let mut nvalids1 = 0;
    let mut nvalids2 = 0;
    for raw_passport in raw_passports {
        let re = Regex::new(r"(\w{3}):([#\w\d]+)").unwrap();
        let passport: HashMap<_, _> = re
            .find_iter(raw_passport)
            .map(|mat| {
                let cap = re.captures(mat.as_str()).unwrap();
                (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str())
            })
            .collect();
       
        //dbg!(&passport);
        if is_valid(&passport, false) {
            nvalids1 += 1;
        }
        if is_valid(&passport, true) {
            nvalids2 += 1;
        }
    }
    println!("Parte 1: nvalids={}", nvalids1);
    println!("Parte 2: nvalids={}", nvalids2);
}
