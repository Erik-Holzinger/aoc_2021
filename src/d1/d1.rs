use std::fs;

pub fn count_larger_measurements() -> u32 {
    let input = fs::read_to_string("d1/input.txt").expect("Error reading file");
    let array: Vec<u32> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut counter: u32 = 0;
    array.windows(2)
        .for_each(|w| if w[1] > w[0] {counter += 1});
    counter
}

pub fn count_larger_measurements_windows() -> u32 {
    let input = fs::read_to_string("d1/input.txt").expect("Error reading file");
    let array: Vec<u32> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut counter: u32 = 0;
    array.windows(3)
        .zip(array.windows(3).skip(1))
        .for_each(|(w0, w1)| if w1.iter().sum::<u32>() > w0.iter().sum::<u32>() {counter += 1});
    counter
}