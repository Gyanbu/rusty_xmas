use rusty_xmas;

fn fuel_usage_part_1(subs: &Vec<u32>, position: &u32) -> u32 {
    let mut fuel_used = 0;
    for sub in subs {
        if sub < position {
            fuel_used += position - sub;
        } else {
            fuel_used += sub - position;
        }
    }
    fuel_used
}

fn fuel_usage_part_2(subs: &Vec<u32>, position: &u32) -> u32 {
    let mut fuel_used = 0;
    for sub in subs {
        let distance = sub.max(position) - sub.min(position);
        fuel_used += (distance + 1) * distance / 2;
    }
    fuel_used
}

fn main() {
    let input = rusty_xmas::load_input!();
    let mut subs: Vec<u32> = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    subs.sort();

    let best_position: u32 = (subs[subs.len() / 2 - 1] + subs[subs.len() / 2]) / 2;
    println!("Part 1: {}", fuel_usage_part_1(&subs, &best_position));

    let min_pos = subs[0];
    let max_pos = subs[subs.len() - 1];
    let mut best_pos = fuel_usage_part_2(&subs, &min_pos);
    for pos in min_pos + 1..max_pos {
        best_pos = fuel_usage_part_2(&subs, &pos).min(best_pos);
    }
    println!("Part 2: {}", best_pos);
}
