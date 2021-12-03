use aoc_2021::days;

fn main() {
    let day: u32 = 1;

    match day {
        1 => {
            println!("Single measurements: {}", days::d1::count_larger_measurements());
            println!("Measurement windows: {}", days::d1::count_larger_measurements_windows());
        }
        _ => print!("Day does not exist"),
    }
}
