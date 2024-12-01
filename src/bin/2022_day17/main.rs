use std::collections::HashMap;

use itertools::Itertools;

struct Point {
    x: usize,
    y: usize,
}

struct Tetris {
    // board: Vec<Vec<bool>>,
    wind: Vec<bool>,
}

impl Tetris {
    fn new(wind: Vec<bool>) -> Self {
        Self {
            // board,
            wind,
        }
    }

    fn play(&self, blocks: usize) -> usize {
        let mut board: Vec<[bool; 7]> = vec![];
        let wind_modulo = self.wind.len();

        let mut cache: HashMap<[usize; 2], [usize; 2]> = HashMap::new();

        let mut trunacated: usize = 0;
        let mut y = 0;
        let mut wind_index = 0;

        let mut i = 0;
        while i < blocks {
        // for i in 0..blocks {
            let mut block = match i % 5 {
                0 => {
                    // ----
                    vec![
                        Point { x: 2, y: y + 3 },
                        Point { x: 3, y: y + 3 },
                        Point { x: 4, y: y + 3 },
                        Point { x: 5, y: y + 3 },
                    ]
                }
                1 => {
                    // -|-
                    vec![
                        Point { x: 3, y: y + 3 },
                        Point { x: 2, y: y + 4 },
                        Point { x: 3, y: y + 4 },
                        Point { x: 4, y: y + 4 },
                        Point { x: 3, y: y + 5 },
                    ]
                }
                2 => {
                    // _|
                    vec![
                        Point { x: 2, y: y + 3 },
                        Point { x: 3, y: y + 3 },
                        Point { x: 4, y: y + 3 },
                        Point { x: 4, y: y + 4 },
                        Point { x: 4, y: y + 5 },
                    ]
                }
                3 => {
                    // |
                    vec![
                        Point { x: 2, y: y + 3 },
                        Point { x: 2, y: y + 4 },
                        Point { x: 2, y: y + 5 },
                        Point { x: 2, y: y + 6 },
                    ]
                }
                4 => {
                    // []
                    vec![
                        Point { x: 2, y: y + 3 },
                        Point { x: 3, y: y + 3 },
                        Point { x: 2, y: y + 4 },
                        Point { x: 3, y: y + 4 },
                    ]
                }
                _ => unreachable!(),
            };

            loop {
                let mut can_be_blown = true;
                if !self.wind[wind_index] {
                    if block.iter().min_by_key(|point| point.x).unwrap().x != 0 {
                        for point in &block {
                            // Check if row exists
                            if point.y >= y {
                                continue;
                            }
                            // Check if space is occupied
                            if board[point.y][point.x - 1] {
                                can_be_blown = false;
                                break;
                            }
                        }
                        if can_be_blown {
                            for point in &mut block {
                                point.x -= 1;
                            }
                        }
                    }
                } else {
                    if block.iter().max_by_key(|point| point.x).unwrap().x != 6 {
                        for point in &block {
                            // Check if row exists
                            if point.y >= y {
                                continue;
                            }
                            // Check if space is occupied
                            if board[point.y][point.x + 1] {
                                can_be_blown = false;
                                break;
                            }
                        }
                        if can_be_blown {
                            for point in &mut block {
                                point.x += 1;
                            }
                        }
                    }
                }
                wind_index = (wind_index + 1) % wind_modulo;

                let mut can_fall = true;
                for point in &block {
                    if point.y == 0 {
                        can_fall = false;
                        break;
                    }
                    if point.y - 1 >= y {
                        continue;
                    }
                    if board[point.y - 1][point.x] {
                        can_fall = false;
                        break;
                    }
                }
                if can_fall {
                    for point in &mut block {
                        point.y -= 1;
                    }
                } else {
                    let minmax_block_y = block.iter().minmax_by_key(|point| point.y).into_option().unwrap();
                    for _ in y..=minmax_block_y.1.y {
                        board.push([false; 7]);
                        y += 1;
                    }
                    for point in &block {
                        board[point.y][point.x] = true;
                    }
                    if board[minmax_block_y.0.y].iter().all(|point| *point) {
                        // dbg!(i % 5, wind_index);
                        if let Some(previous) = cache.get(&[i % 5, wind_index]) {
                            let cycle_length = i - previous[0];
                            let height_increase = y + trunacated - previous[1];
                            let remaining_blocks = blocks - i;
                            let cycles_to_skip = remaining_blocks / cycle_length;
                            dbg!(cycle_length, height_increase, remaining_blocks, cycles_to_skip);
                            trunacated += cycles_to_skip * height_increase;
                            i += cycles_to_skip * cycle_length;
                        } else {
                            cache.insert([i % 5, wind_index], [i, y + trunacated]);
                        }
                        board = board.split_off(minmax_block_y.0.y);
                        trunacated += minmax_block_y.0.y;
                        y = board.len();
                    }
                    break;
                }
            }
            // if i % 100000 == 0 {
            //     print!("\r{:.4}%", i as f32 / blocks as f32 * 100f32);
            // }
            i += 1;
        }
        // print_board(&board);
        // println!();

        y + trunacated
    }
}

fn print_board(board: &Vec<[bool; 7]>) {
    let mut i = 10usize;
    for row in board.iter().rev() {
        if i == 0 {
            break;
        }
        i -= 1;
        print!("|");
        for point in row.iter() {
            if *point {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+\n");
}

fn main() {
    let input = rusty_xmas::load_input!();
    let input: Vec<bool> = input.trim_end().chars().map(|char| char == '>').collect();
    // 0 - Left
    // 1 - Right
    let tetris = Tetris::new(input);
    let height = tetris.play(2022);
    println!("Part 1: {}", height);


    let height = tetris.play(1_000_000_000_000);
    println!("Part 2: {}", height);
}
