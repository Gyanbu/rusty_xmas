use std::mem;
use itertools::Itertools;
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

    fn get_region_fence_sale_price(region: &Vec<Point>) -> usize {
        let mut sides = 0;

        // Count horizontal sides
        let mut x_of_up_fence: usize = usize::MAX;
        let mut x_of_down_fence: usize = usize::MAX;
        let mut y: usize = usize::MAX;
        let mut up = false;
        let mut down = false;
        for plot in region.iter().sorted_unstable_by_key(|point| point.y * region.len() + point.x) {
            // println!("{:?}", plot);
            if y != plot.y {
                y = plot.y;
                x_of_up_fence = plot.x;
                x_of_down_fence = plot.x;
                up = false;
                down = false;
            }

            // Check lower bounds
            if let Some(up_neighbor) = plot.add_vector(&VECTORS[0]) {
                // If up is empty...
                if !region.contains(&up_neighbor) {
                    // ...check if it is continuation
                    if !up || plot.x - 1 != x_of_up_fence {
                        up = true;
                        sides += 1;
                    }
                    x_of_up_fence = plot.x;
                } else {
                    up = false;
                }
            } else {
                if !up || plot.x - 1 != x_of_up_fence {
                    up = true;
                    sides += 1;
                }
                x_of_up_fence = plot.x;
            }

            // Check lower bounds
            if let Some(down_neighbor) = plot.add_vector(&VECTORS[2]) {
                // If down is empty...
                if !region.contains(&down_neighbor) {
                    // ...check if it is continuation
                    if !down || plot.x - 1 != x_of_down_fence {
                        down = true;
                        sides += 1;
                    }
                    x_of_down_fence = plot.x;
                } else {
                    down = false;
                }
            } else {
                if !down || plot.x - 1 != x_of_down_fence {
                    down = true;
                    sides += 1;
                }
                x_of_down_fence = plot.x;
            }
        }
        // let DEBUG_horizontal_sides = sides;
        // println!("Horizontal sides: {}", DEBUG_horizontal_sides);
        
        // Count vertical sides
        let mut y_of_left_fence: usize = usize::MAX;
        let mut y_of_right_fence: usize = usize::MAX;
        let mut x: usize = usize::MAX;
        let mut left = false;
        let mut right = false;
        for plot in region.iter().sorted_unstable_by_key(|point| point.x * region.len() + point.y) {
            // println!("{:?}", plot);
            if x != plot.x {
                x = plot.x;
                y_of_left_fence = plot.y;
                y_of_right_fence = plot.y;
                left = false;
                right = false;
            }

            // Check lower bounds
            if let Some(left_neighbor) = plot.add_vector(&VECTORS[3]) {
                // If left is empty...
                if !region.contains(&left_neighbor) {
                    // ...check if it is continuation
                    if !left || plot.y - 1 != y_of_left_fence {
                        left = true;
                        sides += 1;
                    }
                    y_of_left_fence = plot.y;
                } else {
                    left = false;
                }
            } else {
                if !left || plot.y - 1 != y_of_left_fence {
                    left = true;
                    sides += 1;
                }
                y_of_left_fence = plot.y;
            }

            // Check lower bounds
            if let Some(right_neighbor) = plot.add_vector(&VECTORS[1]) {
                // If right is empty...
                if !region.contains(&right_neighbor) {
                    // ...check if it is continuation
                    if !right || plot.y - 1 != y_of_right_fence {
                        right = true;
                        sides += 1;
                    }
                    y_of_right_fence = plot.y;
                } else {
                    right = false;
                }
            } else {
                if !right || plot.y - 1 != y_of_right_fence {
                    right = true;
                    sides += 1;
                }
                y_of_right_fence = plot.y;
            }
        }
        // let DEBUG_vertical_sides = sides - DEBUG_horizontal_sides;
        // println!("Vertical sides: {}", DEBUG_vertical_sides);
        sides * region.len()
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
    for region in &regions {
        answer += Garden::get_region_fence_price(region);
    }
    println!("Part 1: {}", answer);

    let mut answer = 0;
    for region in &regions {
        answer += Garden::get_region_fence_sale_price(region);
    }
    println!("Part 2: {}", answer);
}
