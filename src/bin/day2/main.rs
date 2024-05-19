use rusty_xmas;

fn main() {
    let input = rusty_xmas::load_input!();

    let mut distance: u32 = 0;
    let mut depth: u32 = 0;
    for command in input.lines() {
        match command {
            command if command.contains("forward") => {
                let val: String = command.chars().filter(|c| c.is_numeric()).collect();
                distance += val.parse::<u32>().unwrap();
            }
            command if command.contains("down") => {
                let val: String = command.chars().filter(|c| c.is_numeric()).collect();
                depth += val.parse::<u32>().unwrap();
            }
            command if command.contains("up") => {
                let val: String = command.chars().filter(|c| c.is_numeric()).collect();
                depth -= val.parse::<u32>().unwrap();
            }
            _ => panic!(),
        }
    }
    println!("Part 1: {}", distance * depth);

    let mut aim: u32 = 0;
    let mut distance: u32 = 0;
    let mut depth: u32 = 0;
    for command in input.lines() {
        match command {
            command if command.contains("forward") => {
                let val: String = command.chars().filter(|c| c.is_numeric()).collect();
                let val: u32 = val.parse().unwrap();
                distance += val;
                depth += aim * val;
            }
            command if command.contains("down") => {
                let val: String = command.chars().filter(|c| c.is_numeric()).collect();
                aim += val.parse::<u32>().unwrap();
            }
            command if command.contains("up") => {
                let val: String = command.chars().filter(|c| c.is_numeric()).collect();
                aim -= val.parse::<u32>().unwrap();
            }
            _ => panic!(),
        }
    }
    println!("Part 2: {}", distance * depth);
}
