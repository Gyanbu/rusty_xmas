use std::fmt::Display;

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
    fn process_operations(&self, operations_bits: usize) -> usize {
        let mask = 2usize.pow(self.numbers.len() as u32);
        if operations_bits & mask == mask {
            panic!();
        }
        // println!("{:b}", operations_bits);
        let mut result: usize = self.numbers[0];
        for (n, num) in self.numbers.iter().skip(1).enumerate() {
            let mask = 2usize.pow(n as u32);
            match operations_bits & mask == mask {
                true => {
                    // println!("1");
                    result *= num;
                }
                false => {
                    // println!("0".);
                    result += num;
                }
            }
        }
        // println!();
        result
    }

    fn process_operations_part2(&self, operations: &Vec<usize>) -> usize {
        let mut result: usize = self.numbers[0];
        // println!("{:?}", operations);
        for (n, num) in self.numbers.iter().skip(1).enumerate() {
            match operations[n] {
                0 => {
                    result += num;
                }
                1 => {
                    result *= num;
                }
                2 => {
                    result = result * 10usize.pow(num.ilog10() + 1) + num;
                    // result = (result.to_string() + &num.to_string()).parse().unwrap();
                }
                _ => panic!(),
            }
        }
        // println!("{}\n", result);
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
    for equation in &equations {
        let mask = 2usize.pow(equation.numbers.len() as u32);
        let mut operations: usize = 0;
        while operations & mask == 0 {
            let result = equation.process_operations(operations);
            if result == equation.test_value {
                answer += result;
                break;
            }
            operations += 1;
        }
    }
    println!("Part 1: {}", answer);

    let mut answer: usize = 0;
    for equation in &equations {
        let mut operations = vec![0usize; equation.numbers.len() - 1];
        loop {
            let result = equation.process_operations_part2(&operations);
            // println!("[{}] {:?} {:?} -> {}", equation.test_value, operations, equation.numbers, result);
            if result == equation.test_value {
                answer += result;
                break;
            }
            add_1_to_vec(&mut operations, 3);
            if operations.iter().all(|op| *op == 0) {
                break;
            }
        }
    }
    println!("Part 2: {}", answer);
}

fn add_1_to_vec(vec: &mut Vec<usize>, base: usize) {
    for num in vec.iter_mut().rev() {
        *num = (*num + 1) % base;
        if *num != 0 {
            break;
        }
    }
}
