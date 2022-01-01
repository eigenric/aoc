use itertools::Itertools;

pub fn exercise1() {

    let expenses = vec![1721, 979, 366, 299, 675, 1456];

    let found = expenses.into_iter()
            .combinations(2)
            .filter(|v| v[0] + v[1] == 2020)
            .exactly_one().unwrap();
    let product = found[0] * found[1];
    println!("{} * {} = {}", found[0], found[1], product); 
}
