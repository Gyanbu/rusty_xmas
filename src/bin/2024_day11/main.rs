use std::mem;

fn main() {
    let input = rusty_xmas::load_input!();
    let mut stones: Vec<u128> = input.trim().split_ascii_whitespace().map(|stone| stone.parse().unwrap()).collect();
    
    for i in 0..25 {
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
        println!("{}/25", i + 1);
    }
    println!("Part 1: {}", stones.len());
}