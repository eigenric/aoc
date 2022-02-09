use crate::utils;

pub fn exercise1() {
    let map = utils::load_from_file("input/day3");
    
    let trees = map.iter()
                   .enumerate()
                   .filter( |&(i, x)| x.chars()
                                     .nth(3*i % 31) == Some('#'))
                   .count();
    println!("{}", trees);
}

pub fn part_two() {
    let map = utils::load_from_file("day3");

    let trees1 = map.iter()   
                    .enumerate()
                    .filter( |&(i, x)| x.chars()
                                      .nth(1*i % 31) == Some('#'))
                    .count();
    
    let trees2 = map.iter()      
                    .enumerate()
                    .filter( |&(i, x)| x.chars()
                                      .nth(3*i % 31) == Some('#'))
                    .count();

    let trees3 = map.iter()      
                    .enumerate()
                    .filter( |&(i, x)| x.chars()            
                                      .nth(5*i % 31) == Some('#'))
                    .count();

    let trees4 = map.iter()         
                    .enumerate()
                    .filter( |&(i, x)| x.chars()
                                       .nth(7*i % 31) == Some('#'))
                    .count();


    let trees5 = map.iter()         
                    .step_by(2)
                    .enumerate()
                    .filter( |&(i, x)| x.chars()
                                       .nth(1*i % 31) == Some('#'))
                    .count();

    let product = trees1 * trees2 * trees3 * trees4 * trees5;
    println!("{} * {} * {} * {} * {} = {}", 
            trees1, trees2, 
            trees3, trees4, 
            trees5, product);
}