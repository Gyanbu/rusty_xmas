use rusty_xmas;

fn main() {
    let input = rusty_xmas::load_input!();
    let mut fishes: Vec<u8> = input.trim().chars().filter(|char| char.is_numeric()).map(|int| int.to_digit(10).unwrap() as u8).collect();
    println!("Input: {:?}", fishes);

    for _ in 0..80 {
        // println!("Simulating day {}", day + 1);
        let mut new_fish: usize = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..new_fish {
            fishes.push(8);
        }
    }
    println!("Part 1: {}", fishes.len());


    let fishes: Vec<u8> = input.trim().chars().filter(|char| char.is_numeric()).map(|int| int.to_digit(10).unwrap() as u8).collect();
    let mut fish_tape: [u64; 9] = [0; 9];
    for fish in fishes {
        fish_tape[fish as usize] += 1;
    }
    for _ in 0..256 {
        // println!("Calculating day {}", day + 1);
        let ready_fish = fish_tape[0];
        for i in 0..fish_tape.len() - 1 {
            fish_tape[i] = fish_tape[i + 1];
        }
        fish_tape[6] += ready_fish;
        fish_tape[8] = ready_fish;
    }
    println!("Part 2: {}", fish_tape.iter().sum::<u64>())
}