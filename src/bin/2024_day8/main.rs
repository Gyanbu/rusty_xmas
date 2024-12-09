use std::collections::HashMap;

use itertools::Itertools;
use rusty_xmas::Point;

fn main() {
    let input = rusty_xmas::load_input!();
    let input: Vec<&str> = input.lines().collect();

    let mut location_ids: HashMap<char, usize> = HashMap::new();
    {
        let mut id = 0usize;
        for c in input.iter().flat_map(|line| line.chars()) {
            if c != '.' && !location_ids.contains_key(&c) {
                location_ids.insert(c, id);
                id += 1;
            }
        }
    }
    let mut beacons: Vec<Vec<Point>> = vec![Vec::new(); location_ids.len()];
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if let Some(k) = location_ids.get(&c) {
                beacons[*k].push(Point::new(x, y));
            }
        }
    }
    let map_width = input[0].len();
    let map_height = input.len();

    let mut antinode_map = vec![vec![false; map_width]; map_height];
    for resonating_beacons in beacons.iter() {
        for beacon_pair in resonating_beacons
            .iter()
            .combinations_with_replacement(2)
            .filter(|vec| vec[0] != vec[1])
        {
            let mut vector = beacon_pair[0].vector_between_points(&beacon_pair[1]);
            if let Some(possible_antinode) = beacon_pair[1].add_vector(&vector) {
                if possible_antinode.x < map_width && possible_antinode.y < map_height {
                    antinode_map[possible_antinode.y][possible_antinode.x] = true;
                }
            }
            vector.rotate_180();
            if let Some(possible_antinode) = beacon_pair[0].add_vector(&vector) {
                if possible_antinode.x < map_width && possible_antinode.y < map_height {
                    antinode_map[possible_antinode.y][possible_antinode.x] = true;
                }
            }
        }
    }
    // for row in &antinode_map {
    //     for point in row {
    //         if *point {
    //             print!("O");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    let answer = antinode_map.iter().flatten().filter(|val| **val).count();
    println!("Part 1: {}", answer);

    let mut antinode_map = vec![vec![false; map_width]; map_height];
    for resonating_beacons in beacons.iter() {
        for beacon_pair in resonating_beacons
            .iter()
            .combinations_with_replacement(2)
            .filter(|vec| vec[0] != vec[1])
        {
            antinode_map[beacon_pair[0].y][beacon_pair[0].x] = true;
            antinode_map[beacon_pair[1].y][beacon_pair[1].x] = true;

            let mut vector = beacon_pair[0].vector_between_points(&beacon_pair[1]);

            let mut next_location = Some(beacon_pair[1].clone());
            loop {
                next_location = next_location.unwrap().add_vector(&vector);
                if let Some(possible_antinode) = next_location {
                    if possible_antinode.x < map_width && possible_antinode.y < map_height {
                        next_location = Some(Point::new(possible_antinode.x, possible_antinode.y));
                        antinode_map[possible_antinode.y][possible_antinode.x] = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            vector.rotate_180();

            let mut next_location = Some(beacon_pair[0].clone());
            loop {
                next_location = next_location.unwrap().add_vector(&vector);
                if let Some(possible_antinode) = next_location {
                    if possible_antinode.x < map_width && possible_antinode.y < map_height {
                        next_location = Some(Point::new(possible_antinode.x, possible_antinode.y));
                        antinode_map[possible_antinode.y][possible_antinode.x] = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    for row in &antinode_map {
        for point in row {
            if *point {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    let answer = antinode_map.iter().flatten().filter(|val| **val).count();
    println!("Part 2: {}", answer);
}
