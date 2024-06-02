use std::{
    cmp::{max, min},
    fmt,
    thread::sleep,
    time::Duration,
};

use rusty_xmas;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const SAND_DISPENSER: Point = Point { x: 500, y: 0 };
const NEXT_LOCATION: [[isize; 2]; 3] = [[0, 1], [-1, 1], [1, 1]];

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(str: &str) -> Self {
        let (x, y) = str.split_once(",").unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Empty,
    Rock,
    Sand,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Empty => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

struct Simulation {
    board: Vec<Vec<State>>,
}

impl Simulation {
    fn from_instructions(instructions: &Vec<Vec<Point>>) -> Self {
        let mut board = vec![vec![State::Empty; WIDTH]; HEIGHT];

        for instruction in instructions {
            for points in instruction.windows(2) {
                if points[0].x == points[1].x {
                    let x = points[0].x;
                    let start = min(points[0].y, points[1].y);
                    let end = max(points[0].y, points[1].y);
                    for y in start..=end {
                        board[y][x] = State::Rock;
                    }
                } else if points[0].y == points[1].y {
                    let y = points[0].y;
                    let start = min(points[0].x, points[1].x);
                    let end = max(points[0].x, points[1].x);
                    for x in start..=end {
                        board[y][x] = State::Rock;
                    }
                } else {
                    panic!();
                }
            }
        }
        Self { board }
    }

    fn pour_sand(&mut self) -> bool {
        let mut sand = SAND_DISPENSER;
        if self.board[sand.y][sand.x] == State::Sand {
            return false;
        }
        'outer: loop {
            if sand.y == HEIGHT - 1 {
                return false;
            }
            for next in NEXT_LOCATION {
                if self.board[sand.y.checked_add_signed(next[1]).unwrap()]
                    [sand.x.checked_add_signed(next[0]).unwrap()]
                    == State::Empty
                {
                    sand.x = sand.x.checked_add_signed(next[0]).unwrap();
                    sand.y = sand.y.checked_add_signed(next[1]).unwrap();
                    continue 'outer;
                }
            }
            self.board[sand.y][sand.x] = State::Sand;
            return true;
        }
    }
}

fn main() {
    let input = rusty_xmas::load_input!();
    let mut rock_instructions: Vec<Vec<Point>> = input
        .lines()
        .map(|rocks| {
            rocks
                .split(" -> ")
                .map(|num| Point::from_str(num))
                .collect()
        })
        .collect();

    let mut answer: u32 = 0;
    let mut simulation = Simulation::from_instructions(&rock_instructions);
    while simulation.pour_sand() {
        answer += 1;
    }
    println!("Part 1: {}", answer);

    let max_y = rock_instructions
        .iter()
        .flatten()
        .max_by_key(|point| point.y)
        .unwrap()
        .y;
    let floor_y = max_y + 2;
    rock_instructions.push(vec![
        Point { x: 0, y: floor_y },
        Point {
            x: WIDTH - 1,
            y: floor_y,
        },
    ]);
    let mut answer: u32 = 0;
    let mut simulation = Simulation::from_instructions(&rock_instructions);
    while simulation.pour_sand() {
        answer += 1;
    }
    println!("Part 2: {}", answer);
}
