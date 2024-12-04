struct Board {
    board: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(board: Vec<Vec<u8>>) -> Board {
        Board {
            width: board[0].len(),
            height: board.len(),
            board,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.board.iter()
    }

    fn count_words(&self, x: usize, y: usize) -> u8 {
        const VECTORS: [[isize; 2]; 8] = [
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
            [-1, -1],
            [-1, 0],
            [-1, 1],
        ];

        let mut words = 0;
        'vectors: for vector in VECTORS {
            match x.checked_add_signed(vector[0] * 3) {
                Some(x) => {
                    if x >= self.width {
                        continue;
                    }
                }
                None => continue,
            }
            match y.checked_add_signed(vector[1] * 3) {
                Some(y) => {
                    if y >= self.height {
                        continue;
                    }
                }
                None => continue,
            }
            unsafe {
                for i in 1..4 {
                    let c = self.board[y
                        .checked_add_signed(vector[1] * i as isize)
                        .unwrap_unchecked()][x
                        .checked_add_signed(vector[0] * i as isize)
                        .unwrap_unchecked()];
                    if c != i {
                        continue 'vectors;
                    }
                }
            }
            words += 1;
        }
        words
    }

    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
            return false;
        }

        const VECTORS: [[isize; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let mut neighbors = Vec::new();
        unsafe {
            for vector in VECTORS {
                neighbors.push(
                    self.board[y.checked_add_signed(vector[1]).unwrap_unchecked()]
                        [x.checked_add_signed(vector[0]).unwrap_unchecked()],
                );
            }
        }
        for i in 0..4 {
            if neighbors[i % 4] == 1
                && neighbors[(i + 1) % 4] == 1
                && neighbors[(i + 2) % 4] == 3
                && neighbors[(i + 3) % 4] == 3
            {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input = rusty_xmas::load_input!();

    let board: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => 0,
                    'M' => 1,
                    'A' => 2,
                    'S' => 3,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // for line in &board {
    //     println!("{line:?}");
    // }
    let board = Board::new(board);

    let mut answer: usize = 0;
    for (y, row) in board.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 0 {
                answer += board.count_words(x, y) as usize;
            }
        }
    }
    println!("Part 1: {}", answer);

    let mut answer: usize = 0;
    for (y, row) in board.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 2 {
                answer += board.is_x_mas(x, y) as usize;
            }
        }
    }
    println!("Part 2: {}", answer);
}
