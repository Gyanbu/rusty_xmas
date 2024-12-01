use itertools::Itertools;
use serde_json::value;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
struct Valve {
    id: u8,
    flow_rate: u32,
    tunnels: Vec<u8>,
}

impl Valve {
    fn is_pressurized(&self) -> bool {
        self.flow_rate != 0
    }
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] flow rate: {} -> {:?}",
            self.id, self.flow_rate, self.tunnels
        )
    }
}

struct Volcano {
    valves: Vec<Valve>,
    paths: Vec<Vec<u32>>,
}

impl Volcano {
    fn new(valves: Vec<Valve>) -> Self {
        // let mut important_nodes: Vec<u8> = Vec::new();
        // for valve in &valves {
        //     if valve.id == 0 || valve.is_pressurized() {
        //         important_nodes.push(valve.id);
        //     }
        // }

        let mut paths = vec![vec![0u32; valves.len()]; valves.len()];
        for valve in &valves {
            let valve_id = valve.id;
            let mut visited_valves: Vec<bool> = vec![false; valves.len()];
            visited_valves[valve_id as usize] = true;

            let mut time = 1u32;
            let mut tunnels = valves[valve_id as usize].tunnels.clone();
            while !tunnels.is_empty() {
                let mut new_tunnels: Vec<u8> = Vec::new();
                for tunnel in tunnels.drain(..) {
                    if visited_valves[tunnel as usize] {
                        continue;
                    }
                    visited_valves[tunnel as usize] = true;
                    paths[valve_id as usize][tunnel as usize] = time;
                    new_tunnels.extend_from_slice(&valves[tunnel as usize].tunnels);
                }
                tunnels = new_tunnels;
                time += 1;
            }
        }
        Self { valves, paths }
    }

    // First attempt, didn't work obviously
    //
    fn best_flow_bruteforce(&self) -> u32 {
        let mut pressurized_valves: Vec<u8> = Vec::new();
        for valve in &self.valves {
            if valve.is_pressurized() {
                pressurized_valves.push(valve.id);
            }
        }
        let mut best_order: Vec<&u8> = Vec::new();

        let mut best_flow = 0u32;
        for (i, order) in pressurized_valves
            .iter()
            .permutations(pressurized_valves.len())
            .enumerate()
        {
            // print!("\r{}", i);
            let mut time = 30u32;
            let mut flow = 0u32;
            let mut location = 0u8;

            for valve in &order {
                let valve = &self.valves[**valve as usize];
                time = time.saturating_sub(self.paths[location as usize][valve.id as usize] + 1);
                if time == 0 {
                    break;
                }
                flow += time * valve.flow_rate;
                location = valve.id;
            }

            if best_flow < flow {
                best_flow = flow;
                best_order = order;
                // for valve in order {
                //     println!("{}", &self.valves[*valve as usize])
                // }
                // println!();
            }
        }
        dbg!(best_order);
        best_flow
    }

    fn best_flow(&self) -> u32 {
        let mut pressurized_valves: Vec<u8> = Vec::new();
        for valve in &self.valves {
            if valve.is_pressurized() {
                pressurized_valves.push(valve.id);
            }
        }
        pressurized_valves.sort_by_key(|valve| &self.valves[*valve as usize].flow_rate);
        // pressurized_valves.reverse();

        let mut order: Vec<u8> = Vec::new();
        let mut best_flow = 0u32;
        while !pressurized_valves.is_empty() {
            let valve = pressurized_valves.pop().unwrap();
            let valve = &self.valves[valve as usize];

            let mut new_order: Option<Vec<u8>> = None;
            for i in 0..=order.len() {
                let mut order_clone = order.clone();
                order_clone.insert(i, valve.id);

                let mut time = 30u32;
                let mut flow = 0u32;
                let mut location = 0u8;
                for valve in &order_clone {
                    let valve = &self.valves[*valve as usize];
                    time =
                        time.saturating_sub(self.paths[location as usize][valve.id as usize] + 1);
                    if time == 0 {
                        break;
                    }
                    flow += time * valve.flow_rate;
                    location = valve.id;
                }

                if flow >= best_flow {
                    best_flow = flow;
                    new_order = Some(order_clone.clone());
                }
            }
            match new_order {
                Some(new_order) => order = new_order,
                None => panic!(),
            }
        }
        dbg!(order);
        best_flow
    }
}

fn main() {
    let input = rusty_xmas::load_input!();

    let mut name_to_id = HashMap::new();
    name_to_id.insert("AA", 0);
    let mut i: u8 = 1;
    for valve in input.lines() {
        let name = valve.get(6..8).unwrap();
        if name == "AA" {
            continue;
        }
        name_to_id.insert(name, i as u8);
        i += 1;
    }

    let mut valves: Vec<Valve> = Vec::new();
    for valve in input.lines() {
        let name = valve.get(6..8).unwrap();
        let flow_rate: u32 = valve
            .chars()
            .filter(|char| char.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        let tunnels: Vec<u8> = valve
            .split_once("valve")
            .unwrap()
            .1
            .trim_start_matches(['s', ' '])
            .split(", ")
            .map(|str| *name_to_id.get(str).unwrap())
            .collect();
        valves.push(Valve {
            id: *name_to_id.get(name).unwrap(),
            flow_rate,
            tunnels,
        })
    }
    valves.sort_by_key(|valve| valve.id);

    let volcano = Volcano::new(valves);
    // let best_flow_bruteforce = volcano.best_flow_bruteforce();
    // println!("Part 1: {}", best_flow_bruteforce);
    let best_flow = volcano.best_flow();
    println!("Part 1: {}", best_flow);
}
