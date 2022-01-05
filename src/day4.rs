use regex::Regex;
use std::fs;
use std::collections::HashMap;

pub fn is_equal(v1: &Vec<&str>, v2: &Vec<&str>) -> bool {
    let matching = v1.iter()
                     .zip(v2.iter())
                     .filter(|&(a,b)| a == b)
                     .count();
    //println!("Coincidencias: {}", matching);
    matching == v1.len() 
}

pub fn is_valid(passport: &HashMap<&str, &str>, inner: bool) -> bool {
    let keys: Vec<&str> = vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];
    let mut passport_keys: Vec<_> = passport.keys().copied().collect();
    passport_keys.sort();

    let valid_keys = match passport_keys.len() {
        7 => !passport_keys.contains(&"cid"),
        8 => is_equal(&passport_keys, &keys),
        _ => false,
    };

    //println!("Valid keys: {}", valid_keys);
    if inner {
        valid_keys && has_valid_values(passport)
    } else {
        valid_keys
    }
}

pub fn validate_byr(value: &str) -> bool {
    let year: i32 = value.parse::<i32>().unwrap();
    1920 <= year && year <= 2002
}

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

pub fn validate_eyr(value: &str) -> bool {
    let year: i32 = value.parse::<i32>().unwrap();
    2020 <= year && year <= 2030
}

pub fn validate_hcl(value: &str) -> bool {
    let re = Regex::new(r"#[\da-f]{6}").unwrap();
    re.is_match(value)
}

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

pub fn validate_iyr(value: &str) -> bool {
    let year: i32 = value.parse::<i32>().unwrap();
    2010 <= year && year <= 2020
}

pub fn validate_pid(value: &str) -> bool {
    let svalue = String::from(value);
    svalue.chars().all(|c| c.is_numeric()) && svalue.len() == 9
}

pub fn validate_key(key: &str, value: &str) -> bool{
    match key {
            "byr" => validate_byr(value),
            "cid" => true,
            "ecl" => validate_ecl(value),
            "eyr" => validate_eyr(value),
            "hcl" => validate_hcl(value),
            "hgt" => validate_hgt(value),
            "iyr" => validate_iyr(value),
            "pid" => validate_pid(value),
            _ => false,
    }
}

pub fn has_valid_values(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().map(|(&key, value)| validate_key(key, value)).all(|x| x)
}

pub fn exercise1() {
    let input: String = fs::read_to_string("day4").expect("No se pudo leer");
    let raw_passports: Vec<&str> = input.split("\n\n").collect();

    let mut nvalids = 0;
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
        if is_valid(&passport, true) {
            nvalids += 1;
        }
    }
    println!("El numero de pasaportes validos es {}", nvalids);
}