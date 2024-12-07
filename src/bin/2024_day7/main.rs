use std::{fmt::Display, ops::BitAnd};

#[derive(Debug)]
struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.test_value,
            self.numbers
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Equation {
    fn process_operations(&self, operations_bits: u16) -> usize {
        let mask = 2u16.pow(self.numbers.len() as u32);
        if operations_bits & mask == mask {
            panic!();
        }
        // println!("{:b}", operations_bits);
        let mut result: usize = self.numbers[0];
        for (n, num) in self.numbers.iter().skip(1).enumerate() {
            let mask = 2u16.pow(n as u32);
            match operations_bits & mask == mask {
                true => {
                    // println!("1");
                    result *= num;
                },
                false => {
                    // println!("0".);
                    result += num;
                },
            }
        }
        // println!();
        result
    }
}

fn main() {
    let input = rusty_xmas::load_input!();
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|char| *char != ':')
                .collect::<String>()
                .split_ascii_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();
    let mut equations = Vec::new();
    for mut equation in input {
        let numbers = equation.split_off(1);
        equations.push(Equation {
            test_value: equation[0],
            numbers,
        });
    }

    let mut answer: usize = 0;
    'equations: for equation in equations {
        let DEBUG = equation.test_value;
        let mask = 2u16.pow(equation.numbers.len() as u32);
        let mut operations: u16 = 0;
        while operations & mask == 0 {
            let result = equation.process_operations(operations);
            if result == equation.test_value {
                answer += result;
                continue 'equations;
            }
            operations += 1;
        }
    }
    println!("Part 1: {}", answer);
}