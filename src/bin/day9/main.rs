use rusty_xmas;

fn _print_board(board: &Vec<Vec<u8>>) {
    let width = board[0].len();
    let height = board.len();

    for y in 0..height {
        for x in 0..width {
            print!("{} ", board[y][x]);
        }
        println!();
    }
}

fn get_neigbors(
    board: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    const NEIGHBORS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let width = board[0].len() as i32;
    let height = board.len() as i32;

    NEIGHBORS.iter().filter_map(move |&(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < width && ny < height {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    })
}

fn get_basin_size(board: &Vec<Vec<u8>>, lowest_point: (usize, usize)) -> u32 {
    let mut edge_points: Vec<(usize, usize)> = vec![lowest_point];

    let mut board_mask = board.clone();
    for row in board_mask.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 0;
        }
    }
    board_mask[lowest_point.1][lowest_point.0] = 1;

    while !edge_points.is_empty() {
        let (x, y) = edge_points.pop().unwrap();
        for (nx, ny) in get_neigbors(board, x, y) {
            if board[y][x] < board[ny][nx] && board[ny][nx] != 9 {
                edge_points.push((nx, ny));
                board_mask[ny][nx] = 1;
            }
        }
    }

    // _print_board(&board_mask);
    let basin_size = board_mask
        .iter()
        .flatten()
        .filter(|cell| cell == &&1)
        .count();
    basin_size as u32
}

fn main() {
    let input = rusty_xmas::load_input!();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    // println!("{}x{}", width, height)

    let mut board: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for (y, row) in input.lines().enumerate() {
        for (x, num) in row
            .trim()
            .chars()
            .map(|char| char.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            board[y][x] = num;
        }
    }
    // print_board(&board);

    let mut risk_level: u32 = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for y in 0..height {
        'board_iter: for x in 0..width {
            let point = board[y][x];
            for neighbor in get_neigbors(&board, x, y) {
                if point >= board[neighbor.1][neighbor.0] {
                    continue 'board_iter;
                }
            }
            risk_level += point as u32 + 1;
            low_points.push((x, y));
        }
    }
    println!("Part 1: {}", risk_level);

    let mut basins: Vec<u32> = Vec::with_capacity(low_points.len());
    for point in low_points {
        let basin_size = get_basin_size(&board, point);
        basins.push(basin_size);
    }
    basins.sort();
    // println!("{}, {}. {}", basins[basins.len() - 1], basins[basins.len() - 2], basins[basins.len() - 3]);
    println!(
        "part 2: {}",
        basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3]
    );
}
