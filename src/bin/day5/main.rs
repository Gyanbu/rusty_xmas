use rusty_xmas;

fn main() {
    let input = rusty_xmas::load_input!();

    let instructions: Vec<(&str, &str)> = input
    .lines()
    .map(|line| line.trim().split_once(" -> ").unwrap())
    .collect();

    let instructions: Vec<Vec<[u32; 2]>> = instructions
        .iter()
        .map(|(first, second)| {
            let first_parsed: Vec<_> = first.split(',').map(|pos| pos.parse::<u32>().unwrap()).collect();
            let second_parsed: Vec<_> = second.split(',').map(|pos| pos.parse::<u32>().unwrap()).collect();
            vec![[
                first_parsed[0],
                first_parsed[1],
            ], [
                second_parsed[0],
                second_parsed[1],
            ]]
        }).collect();
    println!("Instructions: {:?}", instructions);
    
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    for instruction in &instructions {
        for point in instruction {
            width = width.max(point[0]);
            height = height.max(point[1]);
        }
    }
    width += 1;
    height += 1;
    let mut board: Vec<Vec<u32>> = vec![vec![0; height as usize]; width as usize];

    for instruction in &instructions {
        let start = instruction[0]; 
        let end = instruction[1]; 

        if start[0] == end[0] {
            for y in start[1].min(end[1])..=start[1].max(end[1]) {
                board[start[0] as usize][y as usize] += 1;
            }
        } else if start[1] == end[1] {
            for x in start[0].min(end[0])..=start[0].max(end[0]) {
                board[x as usize][start[1] as usize] += 1;
            }
        } else {
            continue;
        }
    }
    let overlaps = board.iter().flat_map(|col| col.iter()).filter(|val| **val > 1).count();
    println!("Part 1: {}", overlaps);


    let mut board: Vec<Vec<u32>> = vec![vec![0; height as usize]; width as usize];

    for instruction in &instructions {
        let start = instruction[0]; 
        let end = instruction[1]; 

        if start[0] == end[0] {
            for y in start[1].min(end[1])..=start[1].max(end[1]) {
                board[start[0] as usize][y as usize] += 1;
            }
        } else if start[1] == end[1] {
            for x in start[0].min(end[0])..=start[0].max(end[0]) {
                board[x as usize][start[1] as usize] += 1;
            }
        } else {
            let x_offset = if start[0] < end[0] {
                1
            } else {
                -1
            };
            let y_offset = if start[1] < end[1] {
                1
            } else {
                -1
            };
            for (i, _) in (start[0].min(end[0])..=start[0].max(end[0])).enumerate() {
                board[(start[0] as i32 + i as i32  * x_offset) as usize][(start[1] as i32 + i as i32 * y_offset) as usize] += 1;
            }
        }
    }
    let overlaps = board.iter().flat_map(|col| col.iter()).filter(|val| **val > 1).count();
    println!("Part 2: {}", overlaps);
}