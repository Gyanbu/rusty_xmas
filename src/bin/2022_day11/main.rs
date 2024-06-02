use std::ops::{AddAssign, MulAssign};

enum Operation {
    Add(u32),
    Multiply(u32),
    Power,
}

struct Monkey {
    inspections: usize,
    items: Vec<Item>,
    test: [usize; 3],
    operation: Operation,
}

struct Item {
    mod_2: u32,
    mod_3: u32,
    mod_5: u32,
    mod_7: u32,
    mod_11: u32,
    mod_13: u32,
    mod_17: u32,
    mod_19: u32,
    mod_23: u32,
}

impl Item {
    fn new(val: u32) -> Self {
        Item {
            mod_2: val % 2,
            mod_3: val % 3,
            mod_5: val % 5,
            mod_7: val % 7,
            mod_11: val % 11,
            mod_13: val % 13,
            mod_17: val % 17,
            mod_19: val % 19,
            mod_23: val % 23,
        }
    }

    fn is_dividable(&self, modulo: usize) -> bool {
        match modulo {
            2 => self.mod_2 == 0,
            3 => self.mod_3 == 0,
            5 => self.mod_5 == 0,
            7 => self.mod_7 == 0,
            11 => self.mod_11 == 0,
            13 => self.mod_13 == 0,
            17 => self.mod_17 == 0,
            19 => self.mod_19 == 0,
            23 => self.mod_23 == 0,
            _ => panic!(),
        }
    }

    fn power(&mut self) {
        self.mod_2 = self.mod_2.pow(2) % 2;
        self.mod_3 = self.mod_3.pow(2) % 3;
        self.mod_5 = self.mod_5.pow(2) % 5;
        self.mod_7 = self.mod_7.pow(2) % 7;
        self.mod_11 = self.mod_11.pow(2) % 11;
        self.mod_13 = self.mod_13.pow(2) % 13;
        self.mod_17 = self.mod_17.pow(2) % 17;
        self.mod_19 = self.mod_19.pow(2) % 19;
        self.mod_23 = self.mod_23.pow(2) % 23;
    }
}

impl AddAssign<u32> for Item {
    fn add_assign(&mut self, rhs: u32) {
        self.mod_2 = (self.mod_2 + rhs) % 2;
        self.mod_3 = (self.mod_3 + rhs) % 3;
        self.mod_5 = (self.mod_5 + rhs) % 5;
        self.mod_7 = (self.mod_7 + rhs) % 7;
        self.mod_11 = (self.mod_11 + rhs) % 11;
        self.mod_13 = (self.mod_13 + rhs) % 13;
        self.mod_17 = (self.mod_17 + rhs) % 17;
        self.mod_19 = (self.mod_19 + rhs) % 19;
        self.mod_23 = (self.mod_23 + rhs) % 23;
    }
}

impl MulAssign<u32> for Item {
    fn mul_assign(&mut self, rhs: u32) {
        self.mod_2 = (self.mod_2 * rhs) % 2;
        self.mod_3 = (self.mod_3 * rhs) % 3;
        self.mod_5 = (self.mod_5 * rhs) % 5;
        self.mod_7 = (self.mod_7 * rhs) % 7;
        self.mod_11 = (self.mod_11 * rhs) % 11;
        self.mod_13 = (self.mod_13 * rhs) % 13;
        self.mod_17 = (self.mod_17 * rhs) % 17;
        self.mod_19 = (self.mod_19 * rhs) % 19;
        self.mod_23 = (self.mod_23 * rhs) % 23;
    }
}

impl Monkey {
    fn new(items: Vec<u32>, test: [usize; 3], operation: Operation) -> Self {
        let items = items.iter().map(|item| Item::new(*item)).collect();
        Monkey {
            inspections: 0,
            items,
            test,
            operation,
        }
    }

    fn update_items(&mut self) {
        self.inspections += self.items.len();
        for i in 0..self.items.len() {
            item_operation(&self.operation, &mut self.items[i]);
        }
    }

    fn throw_items(&mut self) -> Vec<(usize, Item)> {
        self.update_items();

        let buf = std::mem::take(&mut self.items);
        buf.into_iter()
            .map(|item| (self.test(&item), item))
            .collect()
    }

    fn test(&self, item: &Item) -> usize {
        if item.is_dividable(self.test[0]) {
            self.test[1]
        } else {
            self.test[2]
        }
    }
}

fn item_operation(operation: &Operation, item: &mut Item) {
    match operation {
        Operation::Add(value) => *item += *value,
        Operation::Multiply(value) => *item *= *value,
        Operation::Power => item.power(),
    }
}

fn main() {
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");

    let mut monkeys: [Monkey; 8] = [
        Monkey::new(vec![73, 77], [11, 6, 5], Operation::Multiply(5)),
        Monkey::new(vec![57, 88, 80], [19, 6, 0], Operation::Add(5)),
        Monkey::new(
            vec![61, 81, 84, 69, 77, 88],
            [5, 3, 1],
            Operation::Multiply(19),
        ),
        Monkey::new(
            vec![78, 89, 71, 60, 81, 84, 87, 75],
            [3, 1, 0],
            Operation::Add(7),
        ),
        Monkey::new(
            vec![60, 76, 90, 63, 86, 87, 89],
            [13, 2, 7],
            Operation::Add(2),
        ),
        Monkey::new(vec![88], [17, 4, 7], Operation::Add(1)),
        Monkey::new(vec![84, 98, 78, 85], [7, 5, 4], Operation::Power),
        Monkey::new(vec![98, 89, 78, 73, 71], [2, 3, 2], Operation::Add(4)),
    ];

    // let mut monkeys: [Monkey; 4] = [
    //     Monkey::new(vec![79, 98], [23, 2, 3], Operation::Multiply(19)),
    //     Monkey::new(vec![54, 65, 75, 74], [19, 2, 0], Operation::Add(6)),
    //     Monkey::new(vec![79, 60, 97], [13, 1, 3], Operation::Power),
    //     Monkey::new(vec![74], [17, 0, 1], Operation::Add(3)),
    // ];

    for _round in 0u32..10000 {
        // println!("Round {}", round);
        for monkey in 0..monkeys.len() {
            // println!("Test: {:?}", monkeys[monkey].test);
            let items = monkeys[monkey].throw_items();
            for (i, item) in items {
                // println!("{} -> {}", monkey, i);
                monkeys[i].items.push(item);
            }
            // print!("{} ", monkeys[monkey].inspections);
            // println!();
        }
    }

    let mut monkey_business: Vec<usize> = Vec::new();
    for monkey in monkeys {
        monkey_business.push(monkey.inspections);
    }
    monkey_business.sort();
    monkey_business.reverse();
    // println!("{:?}", monkey_business);
    println!("{}", monkey_business[0] * monkey_business[1])
}
