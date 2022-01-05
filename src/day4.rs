use regex::Regex;
use std::fs;

pub fn is_equal(v1: &Vec<&str>, v2: &Vec<&str>) -> bool {
    let matching = v1.iter()
                     .zip(v2.iter())
                     .filter(|&(a,b)| a == b)
                     .count();
    //println!("Coincidencias: {}", matching);
    matching == v1.len() 
}

pub fn is_valid(passport_keys: &Vec<&str>, keys: &Vec<&str>) -> bool {
    match passport_keys.len() {
        7 => !passport_keys.contains(&"cid"),
        8 => is_equal(passport_keys, keys),
        _ => false,
    }
}

pub fn exercise1() {
    let input: String = fs::read_to_string("day4").expect("No se pudo leer");
    let raw_passports: Vec<&str> = input.split("\n\n").collect();
    let keys: Vec<&str> = vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

    let mut nvalids = 0;
    for passport in raw_passports {
        // println!("{:?}\n---", passport);
        let re = Regex::new(r"(\w{3}):").unwrap();
        let mut passport_keys: Vec<&str> = re
            .find_iter(passport)
            .map(|mat| re.captures(mat.as_str()).unwrap().get(1).unwrap().as_str())
            .collect();
        passport_keys.sort();

        // println!("V{:?}", KEYS);
        // println!("P{:?}", passport_keys);

        if is_valid(&passport_keys, &keys) {
            nvalids += 1;
        }
    }
    println!("El numero de pasaportes validos es {}", nvalids);
}
