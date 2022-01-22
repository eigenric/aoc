
pub fn exercise1() {

    let cadenas : Vec<&str> = vec!["abc", "ba", "acb"];
    let cadenas : Vec<String> = cadenas.iter().map(|c| c.to_string()).collect();
    let result = cadenas.into_iter().reduce(|a, b| {
        [a, b].join("")
    });

    println!("{}", result.unwrap());

}