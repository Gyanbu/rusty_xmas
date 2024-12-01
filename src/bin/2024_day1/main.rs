use std::collections::HashMap;

use itertools::{zip_eq, Itertools};

fn main() {
    let input = rusty_xmas::load_input!();
    // let input: Vec<[u32; 2]> = input.lines().map(|line| {
    //   line.split_once("   ").unwrap()
    // }).map(|(num1, num2)| {
    //   [num1.parse::<u32>().unwrap(), num2.parse::<u32>().unwrap()]
    // }).collect();
    // dbg!(input);
    let mut left_list: Vec<u32> = Vec::with_capacity(input.len());
    let mut right_list: Vec<u32> = Vec::with_capacity(input.len());

    for line in input.lines() {
        let (left_location, right_location) = line.split_once("   ").unwrap();
        left_list.push(left_location.parse().unwrap());
        right_list.push(right_location.parse().unwrap());
    }

    let mut answer: u32 = 0;
    for (l_location_id, r_location_id) in
        zip_eq(left_list.iter().sorted(), right_list.iter().sorted())
    {
        answer += l_location_id.abs_diff(*r_location_id);
    }
    println!("Part 1: {}", answer);

    let mut left_map: HashMap<u32, u32> = HashMap::new();
    let mut right_map: HashMap<u32, u32> = HashMap::new();
    for location_id in left_list {
        match left_map.get_mut(&location_id) {
            Some(i) => {
                *i += 1;
            }
            None => {
                left_map.insert(location_id, 1);
            }
        }
    }
    for location_id in right_list {
        match right_map.get_mut(&location_id) {
            Some(i) => {
                *i += 1;
            }
            None => {
                right_map.insert(location_id, 1);
            }
        }
    }
    let mut answer: u32 = 0;
    for (key, i) in left_map {
        let similarity = key * i * right_map.get(&key).unwrap_or(&0);
        answer += similarity;
    }
    println!("Part 2: {}", answer);
}
