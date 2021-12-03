use aoc_2021::d1::d1;
use aoc_2021::d2::d2;

fn main() {
    let day: u32 = 1;

    match day {
        1 => {
            println!("Single measurements: {}", d1::count_larger_measurements());
            println!("Measurement windows: {}", d1::count_larger_measurements_windows());
        }
        _ => print!("Day does not exist"),
    }
}
