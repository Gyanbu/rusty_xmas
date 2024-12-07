use std::collections::HashMap;

fn main() {
    let input: Vec<String> = rusty_xmas::load_input!()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let pos = input.iter().position(|line| line.is_empty()).unwrap();

    let rules = &input[..pos];
    let batches = &input[pos + 1..];

    let mut rules_map: HashMap<u8, Vec<u8>> = HashMap::new();
    for rule in rules {
        let (left, right) = rule.split_once('|').unwrap();
        let (left, right): (u8, u8) = (left.parse().unwrap(), right.parse().unwrap());
        if rules_map.contains_key(&right) {
            rules_map.get_mut(&right).unwrap().push(left);
        } else {
            rules_map.insert(right, vec![left]);
        }
    }

    let batches: Vec<Vec<u8>> = batches
        .into_iter()
        .map(|batch| batch.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();
    
    let mut part1_answer: usize = 0;
    'batches: for batch in &batches {
        let mut printed = Vec::new();
        for page in batch {
            if let Some(rules) = rules_map.get(&page) {
                if !rules.iter().all(|rule| {
                    !batch.contains(rule) || printed.contains(rule)
                }) {
                    continue 'batches;
                }
            }
            printed.push(*page);
        }
        part1_answer += batch[(batch.len() - 1) / 2] as usize;
    }
    println!("Part 1: {}", part1_answer);


    let mut answer: usize = 0;
    for batch in batches {
        let mut batch = batch.clone();
        loop {
            let mut modified = false;
            let mut printed = Vec::new();
            for (p, page) in batch.clone().iter().enumerate() {
                if let Some(rules) = rules_map.get(&page) {
                    if !rules.iter().all(|rule| {
                        !batch.contains(rule) || printed.contains(rule)
                    }) {
                        batch.swap(p, p + 1);
                        modified = true;
                    }
                }
                printed.push(*page);
            }
            if !modified {
                break;
            }
        }
        answer += batch[(batch.len() - 1) / 2] as usize;
    }
    println!("Part 2: {}", answer - part1_answer);
}
