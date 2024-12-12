use std::{collections::HashSet, mem};

use rusty_xmas::{Point, Vector};

struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<u8>>,
}

impl Board {
    fn new(board: Vec<Vec<u8>>) -> Self {
        Self {
            width: board[0].len(),
            height: board.len(),
            board,
        }
    }

    fn get(&self, point: Point) -> Option<u8> {
        if point.x < 0 || point.x > self.width - 1 || point.y < 0 || point.y > self.height - 1 {
            return None;
        };
        Some(self.board[point.y][point.x])
    }

    fn get_trailheads(&self) -> Vec<Point> {
        let mut trailheads = Vec::new();
        for (y, row) in self.board.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == 0 {
                    trailheads.push(Point::new(x, y));
                }
            }
        }
        trailheads
    }

    fn get_neighbors(&self, target: Point, neighbor_height: u8) -> Vec<Point> {
        const VECTORS: &[Vector] = &[
            Vector::new(1, 0),
            Vector::new(0, 1),
            Vector::new(-1, 0),
            Vector::new(0, -1),
        ];
        // let mut neighbors = Vec::new();
        // for vector in VECTORS {
        //     if let Some(neighbor) = target.add_vector(vector) {
        //         if let Some(_) = self.get(neighbor) {
        //             neighbors.push(neighbor);
        //         }
        //     }
        // }
        // neighbors
        VECTORS
            .iter()
            .filter_map(|vector| target.add_vector(&vector))
            .filter(|point| self.get(*point) == Some(neighbor_height))
            .collect()
    }
}

fn main() {
    let input = rusty_xmas::load_input!();

    let board: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let board = Board::new(board);

    let mut answer: usize = 0;
    let trailheads = board.get_trailheads();
    for trailhead in trailheads {
        let mut locations: HashSet<Point> = HashSet::from([trailhead]);
        for height in 1..=9 {
            let mut new_locations: HashSet<Point> = HashSet::new();
            for location in locations.drain() {
                for neighbor in board.get_neighbors(location, height).iter() {
                    new_locations.insert(*neighbor);
                }
            }
            locations = mem::take(&mut new_locations);
        }
        answer += locations.len();
    }
    println!("Part 1: {}", answer);
}
