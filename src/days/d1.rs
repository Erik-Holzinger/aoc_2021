use std::fs;

pub fn count_larger_measurements() -> u32 {
    let input = fs::read_to_string("./src/days/input.txt").expect("Error reading file");
    let array: Vec<u32> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut counter: u32 = 0;
    let _ = array.iter()
        .zip(array.iter().skip(1))
        .inspect(|(a, b)| if b > a {counter += 1})
        .collect::<Vec<_>>();
    counter
}