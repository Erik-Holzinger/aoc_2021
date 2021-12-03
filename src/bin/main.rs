use aoc_2021::days;

fn main() {
    let day: u32 = 1;

    match day {
        1 => {
            println!("{}", days::d1::count_larger_measurements());
        }
        _ => print!("Day does not exist"),
    }
}
