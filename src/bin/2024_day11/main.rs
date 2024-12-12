use std::{collections::HashMap, mem};

fn main() {
    let input = rusty_xmas::load_input!();
    let mut stones: Vec<u128> = input
        .trim()
        .split_ascii_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<u128> = Vec::with_capacity(stones.len());
        for stone in stones.drain(..) {
            if stone == 0 {
                new_stones.push(1);
            } else if stone.ilog10() % 2 == 1 {
                // dbg!(stone);
                let left_stone = stone / 10u128.pow((stone.ilog10() + 1) / 2);
                // dbg!(left_stone);
                let right_stone = stone - left_stone * 10u128.pow((stone.ilog10() + 1) / 2);
                new_stones.append(&mut vec![left_stone, right_stone]);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = mem::take(&mut new_stones);
        // println!("{}/25", i + 1);
    }
    println!("Part 1: {}", stones.len());

    let stones: Vec<u128> = input
        .trim()
        .split_ascii_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    let mut stones_map: HashMap<u128, u128> = HashMap::new();
    for stone in stones {
        match stones_map.get_mut(&stone) {
            Some(ammount) => {
                *ammount += 1;
            }
            None => {
                stones_map.insert(stone, 1);
            }
        }
    }

    for _ in 0..75 {
        let mut new_stones: HashMap<u128, u128> = HashMap::new();
        for (stone, ammount) in stones_map.drain() {
            if stone == 0 {
                match new_stones.get_mut(&1) {
                    Some(val) => {
                        *val += ammount;
                    }
                    None => {
                        new_stones.insert(1, ammount);
                    }
                };
            } else if stone.ilog10() % 2 == 1 {
                let left_stone = stone / 10u128.pow((stone.ilog10() + 1) / 2);
                let right_stone = stone - left_stone * 10u128.pow((stone.ilog10() + 1) / 2);
                match new_stones.get_mut(&left_stone) {
                    Some(val) => {
                        *val += ammount;
                    }
                    None => {
                        new_stones.insert(left_stone, ammount);
                    }
                };
                match new_stones.get_mut(&right_stone) {
                    Some(val) => {
                        *val += ammount;
                    }
                    None => {
                        new_stones.insert(right_stone, ammount);
                    }
                };
            } else {
                match new_stones.get_mut(&(stone * 2024)) {
                    Some(val) => {
                        *val += ammount;
                    }
                    None => {
                        new_stones.insert(stone * 2024, ammount);
                    }
                };
            }
        }
        stones_map = mem::take(&mut new_stones);
        // println!("{}/75", i + 1);
    }
    println!("Part 2: {}", stones_map.into_values().sum::<u128>());
}
