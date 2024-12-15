use std::{mem, thread::sleep_ms};
use rusty_xmas::{Point, Vector};

const VECTORS: &[Vector] = &[
    Vector::new(0, -1),
    Vector::new(1, 0),
    Vector::new(0, 1),
    Vector::new(-1, 0),
];

struct Garden {
    garden: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Garden {
    fn new(garden: Vec<Vec<u8>>) -> Self {
        let width = garden[0].len();
        let height = garden.len();

        Garden {
            garden,
            width,
            height,
        }
    }

    fn get(&self, point: &Point) -> Option<u8> {
        if point.x >= self.width || point.y >= self.height {
            return None;
        }
        // dbg!(point);
        // sleep_ms(1000);
        return Some(self.garden[point.y][point.x]);
    }

    fn get_regions(&self) -> Vec<Vec<Point>> {
        let mut visited_plots = Vec::new();
        let mut regions = Vec::new();
        let mut edges = Vec::new();
        let mut new_edges = Vec::new();
        for (y, row) in self.garden.iter().enumerate() {
            for (x, plant_type) in row.iter().enumerate() {
                let point = Point::new(x, y);
                if visited_plots.contains(&point) {
                    continue;
                }
                visited_plots.push(point);
                let mut region = vec![point];
                edges.push(point);
                while !edges.is_empty() {
                    // println!("{:?}", visited_plots);
                    for edge in edges.drain(..) {
                        for vector in VECTORS {
                            if let Some(new_edge) = edge.add_vector(vector) {
                                if visited_plots.contains(&new_edge) {
                                    continue;
                                }
                                if let Some(plant) = self.get(&new_edge) {
                                    if plant != *plant_type {
                                        continue;
                                    }
                                    visited_plots.push(new_edge);
                                    new_edges.push(new_edge);
                                }
                            }
                        }
                    }
                    edges = mem::take(&mut new_edges);
                    region.append(&mut edges.clone());
                }
                regions.push(region);
            }
        }
        regions
    }

    fn get_region_fence_price(region: &Vec<Point>) -> usize {
        let mut perimeter = region.len() * 4;
        for plot in region {
            for vector in VECTORS {
                if let Some(point) = plot.add_vector(vector) {
                    if region.contains(&point) {
                        perimeter -= 1;
                    }
                }
            }
        }
        perimeter * region.len()
    }
}

fn main() {
    let input = rusty_xmas::load_input!();
    let garden: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 65).collect())
        .collect();

    let garden = Garden::new(garden);
    let regions = garden.get_regions();
    let mut answer = 0;
    for region in regions {
        answer += Garden::get_region_fence_price(&region);
    }
    println!("Part 1: {}", answer);
}
