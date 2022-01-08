use crate::utils;
//use itertools::Itertools;

pub fn to_bin(word: &str, zero: &str, one: &str) -> String {
    word.replace(zero, "0").replace(one, "1")
}

pub fn get_row(seat_row: &str) -> u32 {
    u32::from_str_radix(to_bin(seat_row, "F", "B").as_str(), 2).unwrap()
}

pub fn get_column(seat_column: &str) -> u32 {
    u32::from_str_radix(to_bin(seat_column, "L", "R").as_str(), 2).unwrap()
}

pub fn exercise1() {
    let boarding_pass: Vec<String> = utils::load_from_file("day5");

    let mut seats_ids: Vec<u32> = boarding_pass.iter().map(|seat| {
        let seat_row = &seat[0..7];
        let seat_column = &seat[7..10];

        get_row(seat_row) * 8 + get_column(seat_column)
    }).collect();
    seats_ids.sort();

    let &min = seats_ids.iter().min().unwrap();
    let &max = seats_ids.iter().max().unwrap();

    let all_seats_range: Vec<_> = (min..max).collect();

    let missing_seat = all_seats_range.iter()
                                       .zip(&seats_ids)
                                       .find(|(a,b)| a != b)
                                       .unwrap().0;

                           
    println!("The maximum id is: {}", seats_ids.iter().max().unwrap());
    println!("My seat is: {}", missing_seat);
}