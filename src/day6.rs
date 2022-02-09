use std::collections::HashSet;
use std::fs;

pub fn exercise1() {
    let form: String = fs::read_to_string("day6").expect("No se pudo cargar");
    let groups: Vec<&str> = form.split("\n\n").collect();

    let result : usize = groups.iter().map( |group| {
        let mut group_set : HashSet<char> = group.chars().collect();
        group_set.remove(&'\n');
        group_set.len()
    }).sum();

    println!("Day 6.1: {}", result);
}

pub fn exercise2() {
    let form: String = fs::read_to_string("input/day6").expect("No se pudo cargar");
    let groups: Vec<String> = form.split("\n\n").map(|g| g.to_string()).collect();

    let result : usize = groups.iter().map(|group| {
        // println!("Analizando grupo: ");
        // println!("{}", group);
        let lines : Vec<String> = group.split("\n").map(|g| g.to_string()).collect();
        let count = lines.into_iter().reduce(|a, b| {
            let a_set : HashSet<char> = a.chars().collect();
            let b_set : HashSet<char> = b.chars().collect();
            let in_common : String = a_set.intersection(&b_set).collect();
            if b.len() > 0 { in_common } else { a }
        }).unwrap().len();
        // println!("C: {}", count);
        count
    }).sum();
    println!("Day 6.2: {}", result);
}
