use itertools::Itertools;
use crate::utils;

pub fn exercise1() {
    let expenses: Vec<i64> = utils::loadints_from_file("input/day1");
    let found = expenses.into_iter()
            .combinations(2)
            .filter(|v| v[0] + v[1] == 2020)
            .exactly_one().unwrap();
    let product = found[0] * found[1];
    println!("{} * {} = {}", found[0], found[1], product); 
}

pub fn part_two() {
    
    let expenses: Vec<i64> = utils::loadints_from_file("input/day1");
    let found = expenses.into_iter()
            .combinations(3)
            .filter(|v| v[0] + v[1] + v[2] == 2020)
            .exactly_one().unwrap();
    let product = found[0] * found[1] * found[2];
    println!("{} * {} * {} = {}", found[0], found[1], found[2], product);
}
