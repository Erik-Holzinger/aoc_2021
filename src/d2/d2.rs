use std::fs;

#[derive(Debug)]
struct Cmd {
    pub name: String,
    pub value: u32
}

struct Pos {
    pub aim: u32,
    pub pos: u32,
    pub depth: u32
}

pub fn calculate_position() -> u32 {
    let input = fs::read_to_string("./src/d2/input.txt").expect("Error reading file");
    let array: Vec<Cmd> = input
        .lines()
        .map(|s| s.split(" "))
        .map(|mut split| Cmd {
            name: String::from(split.next().unwrap()),
            value: split.next().unwrap().parse().unwrap()
        })
        .collect();
    let mut pos = Pos {
        aim: 0,
        pos: 0,
        depth: 0
    };
    array.iter()
        .for_each(|s| {
            match s.name.as_str() {
                "forward" => {
                    pos.pos += s.value;
                    pos.depth += pos.aim * s.value;
                },
                "up" => {
                    pos.aim -= s.value;
                },
                "down" => {
                    pos.aim += s.value;
                },
                _ => println!("Scheiße wars")
            }
        });
    pos.pos * pos.depth
}