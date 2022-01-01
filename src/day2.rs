//use itertools::Itertools;
use regex::Regex;

pub fn exercise2() {
    let passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    let re = Regex::new(
            r"(\d)-(\d) (\w): (\w+)"
        ).unwrap();

    let mut nvalids = 0;
    for password in passwords {
        let cap = re.captures(password).unwrap();
        let low: i32 = cap[1].parse::<i32>().unwrap();
        let high: i32 = cap[2].parse::<i32>().unwrap();
        let letter = &cap[3];
        let pass = String::from(&cap[4]);

        let ocurrences = pass.matches(letter).count() as i32;
        if low <= ocurrences && ocurrences <= high {
            println!("{} es valida", pass);
            nvalids += 1;
        } else {
            println!("{} es invalida", pass);
        }
    }
    println!("Hay {} passwords validas", nvalids);
}