use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty = 0,
    Obstacle = 1,
    Guard = 2,
    Visited = 3,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard,
            'X' => Self::Visited,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    delta_x: isize,
    delta_y: isize,
}

impl Vector {
    fn new(delta_x: isize, delta_y: isize) -> Vector {
        Vector { delta_x, delta_y }
    }

    fn rotate_clockwise(&mut self) {
        // 0, 1 -> -1, 0
        // -1, 0 -> 0, -1
        // 0, -1 -> 1, 0
        // 1, 0 -> 0, 1
        let delta_x = -self.delta_y;
        let delta_y = self.delta_x;
        self.delta_x = delta_x;
        self.delta_y = delta_y;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }

    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn add_vector(&self, vector: &Vector) -> Option<Point> {
        let x = self.x.checked_add_signed(vector.delta_x)?;
        let y = self.y.checked_add_signed(vector.delta_y)?;
        Some(Point { x, y })
    }
}

struct Lab {
    board_width: usize,
    board_height: usize,
    board_original: Vec<Vec<Space>>,
    board: Vec<Vec<Space>>,
    guard_location_original: Point,
    guard_location: Point,
    guard_vector: Vector,
}

impl Lab {
    fn new(board: Vec<Vec<char>>) -> Lab {
        let mut guard_location = Point::default();
        let board: Vec<Vec<Space>> = board
            .into_iter()
            .map(|row| row.into_iter().map(|char| Space::from(char)).collect())
            .collect();
        let board_width = board[0].len();
        let board_height = board.len();
        'outer: for (y, row) in board.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if *space == Space::Guard {
                    guard_location = Point::new(x, y);
                    break 'outer;
                }
            }
        }
        Lab {
            board_width,
            board_height,
            board_original: board.clone(),
            board,
            guard_location_original: guard_location.clone(),
            guard_location,
            guard_vector: Vector::new(0, -1),
        }
    }

    fn reset(&mut self) {
        self.board = self.board_original.clone();
        self.guard_location = self.guard_location_original.clone();
        self.guard_vector = Vector::new(0, -1);
    }

    fn step_guard(&mut self) -> bool {
        loop {
            let next_location = self.guard_location.add_vector(&self.guard_vector);
            // Check lower bounds
            if next_location.is_none() {
                self.board[self.guard_location.y][self.guard_location.x] = Space::Visited;
                return false;
            }
            // Check higher bounds
            let next_location = next_location.unwrap();
            if next_location.x >= self.board_width || next_location.y >= self.board_height {
                self.board[self.guard_location.y][self.guard_location.x] = Space::Visited;
                return false;
            }
            // Move forwards
            if self.board[next_location.y][next_location.x] != Space::Obstacle {
                self.board[self.guard_location.y][self.guard_location.x] = Space::Visited;
                self.board[next_location.y][next_location.x] = Space::Guard;
                self.guard_location = next_location;
                return true;
            }
            // Turn right
            self.guard_vector.rotate_clockwise();
            let next_location = self.guard_location.add_vector(&self.guard_vector);
            // Check lower bounds
            if next_location.is_none() {
                self.board[self.guard_location.y][self.guard_location.x] = Space::Visited;
                return false;
            }
        }
    }

    fn simulate_guard(&mut self) -> bool {
        // use std::{thread::sleep, time::Duration};
        // self.print_board();
        let mut guard_visited: HashSet<(Point, Vector)> = HashSet::new();
        while self.step_guard() {
            let guard = (self.guard_location, self.guard_vector);
            if guard_visited.contains(&guard) {
                return false;
            }
            guard_visited.insert(guard);
            // self.print_board();
            // sleep(Duration::from_millis(100));
        }
        // self.print_board();
        true
    }

    fn print_board(&self) {
        for row in &self.board {
            for space in row {
                match space {
                    Space::Empty => print!("."),
                    Space::Obstacle => print!("#"),
                    Space::Guard => match &self.guard_vector {
                        Vector {
                            delta_x: 0,
                            delta_y: 1,
                        } => print!("v"),
                        Vector {
                            delta_x: -1,
                            delta_y: 0,
                        } => print!("<"),
                        Vector {
                            delta_x: 0,
                            delta_y: -1,
                        } => print!("^"),
                        Vector {
                            delta_x: 1,
                            delta_y: 0,
                        } => print!(">"),
                        _ => panic!(),
                    },
                    Space::Visited => print!("X"),
                }
            }
            println!();
        }
        println!();
    }

    fn count_visited_spaces(&self) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|space| **space == Space::Visited)
            .count()
    }

    fn count_loops(&mut self) -> usize {
        let board = self.board.clone();
        let mut loops: usize = 0;
        for (y, row) in board.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                print!(
                    "{}/{}\r",
                    y * self.board_width + x,
                    self.board_width * self.board_height
                );
                if *space == Space::Visited && Point::new(x, y) != self.guard_location_original {
                    self.reset();
                    self.board[y][x] = Space::Obstacle;
                    if !self.simulate_guard() {
                        loops += 1;
                    }
                }
            }
        }
        println!();
        loops
    }
}

fn main() {
    let input = rusty_xmas::load_input!();

    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut lab = Lab::new(board);
    lab.simulate_guard();
    let answer = lab.count_visited_spaces();
    println!("Part 1: {}", answer);

    let answer = lab.count_loops();
    println!("Part 2: {}", answer);
}
