use rusty_xmas;

fn calculate_score(board: &Vec<Vec<u32>>, num: u32) -> u32 {
    println!("{} -> {:?}", num, board);
    let mut score = 0;
    for vec in board {
        for val in vec {
            score += val;
        }
    }
    score * num
}

fn main() {
    let input = rusty_xmas::load_input!();

    let mut input_iter = input.lines();
    let bingo_numbers = input_iter.by_ref().next().unwrap();
    let bingo_numbers: Vec<u32> = bingo_numbers
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    println!("Bingo numbers: {:?}", bingo_numbers);

    let mut horizontal_boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut vertical_boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let input_iter: Vec<u32> = input_iter
        .skip(1)
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|str| str.split_whitespace())
        .flatten()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    for board in input_iter.chunks(25) {
        let mut horizontal_board: Vec<Vec<u32>> = vec![vec![0; 5]; 5];
        let mut vertical_board: Vec<Vec<u32>> = vec![vec![0; 5]; 5];
        for (i, number) in board.into_iter().enumerate() {
            horizontal_board[i / 5][i % 5] = *number;
            vertical_board[i % 5][i / 5] = *number;
        }
        horizontal_boards.push(horizontal_board);
        vertical_boards.push(vertical_board);
    }

    let mut score = 0;
    'outer: for num in &bingo_numbers {
        for i in 0..horizontal_boards.len() {
            for row in &mut horizontal_boards[i] {
                for val_i in (0..row.len()).rev() {
                    if row[val_i] != *num {
                        continue;
                    }
                    row.remove(val_i);
                    if row.is_empty() {
                        score = calculate_score(&horizontal_boards[i], *num);
                        break 'outer;
                    }
                }
            }
            for col in &mut vertical_boards[i] {
                for val_i in (0..col.len()).rev() {
                    if col[val_i] != *num {
                        continue;
                    }
                    col.remove(val_i);
                    if col.is_empty() {
                        score = calculate_score(&vertical_boards[i], *num);
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", score);

    let input_iter = input.lines().skip(1);
    let mut horizontal_boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut vertical_boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let input_iter: Vec<u32> = input_iter
        .skip(1)
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|str| str.split_whitespace())
        .flatten()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    for board in input_iter.chunks(25) {
        let mut horizontal_board: Vec<Vec<u32>> = vec![vec![0; 5]; 5];
        let mut vertical_board: Vec<Vec<u32>> = vec![vec![0; 5]; 5];
        for (i, number) in board.into_iter().enumerate() {
            horizontal_board[i / 5][i % 5] = *number;
            vertical_board[i % 5][i / 5] = *number;
        }
        horizontal_boards.push(horizontal_board);
        vertical_boards.push(vertical_board);
    }

    let mut score = 0;
    'outer: for num in &bingo_numbers {
        'inner: loop {
            for i in (0..horizontal_boards.len()).rev() {
                for row_i in (0..horizontal_boards[i].len()).rev() {
                    for val_i in (0..horizontal_boards[i][row_i].len()).rev() {
                        if horizontal_boards[i][row_i][val_i] != *num {
                            continue;
                        }
                        horizontal_boards[i][row_i].remove(val_i);
                        if horizontal_boards[i][row_i].is_empty() {
                            if horizontal_boards.len() == 1 {
                                score = calculate_score(&horizontal_boards[0], *num);
                                break 'outer;
                            }
                            horizontal_boards.remove(i);
                            vertical_boards.remove(i);
                            continue 'inner;
                        }
                    }
                }
                for col_i in (0..vertical_boards[i].len()).rev() {
                    for val_i in (0..vertical_boards[i][col_i].len()).rev() {
                        if vertical_boards[i][col_i][val_i] != *num {
                            continue;
                        }
                        vertical_boards[i][col_i].remove(val_i);
                        if vertical_boards[i][col_i].is_empty() {
                            if vertical_boards.len() == 1 {
                                score = calculate_score(&vertical_boards[0], *num);
                                break 'outer;
                            }
                            vertical_boards.remove(i);
                            horizontal_boards.remove(i);
                            continue 'inner;
                        }
                    }
                }
            }
            break;
        }
    }
    println!("Part 2: {}", score);
}
