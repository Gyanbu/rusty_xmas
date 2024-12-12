use itertools::Itertools;

fn main() {
    let input = rusty_xmas::load_input!();
    let input: Vec<usize> = input
        .trim_end()
        .chars()
        .map(|num| num.to_digit(10).unwrap() as usize)
        .collect();
    let mut data: Vec<Option<usize>> = Vec::new();

    let mut mode = true;
    let mut file_index: usize = 0;
    for block in &input {
        if mode {
            data.append(&mut vec![Some(file_index); *block]);
            file_index += 1;
        } else {
            data.append(&mut vec![None; *block]);
        }
        mode = !mode;
    }
    let mut l = 0usize;
    let mut r = data.len() - 1;
    while l < r {
        if data[l].is_some() {
            l += 1;
            continue;
        }
        if data[r].is_none() {
            r -= 1;
            continue;
        }
        data.swap(l, r);
        l += 1;
        r -= 1;
    }
    data.truncate(l + 1);
    let answer = get_checksum(data);
    println!("Part 1: {}", answer);

    let mut data: Vec<(Option<usize>, usize)> = Vec::new();
    let mut mode = true;
    let mut file_index: usize = 0;
    for block in input {
        if mode {
            data.push((Some(file_index), block));
            file_index += 1;
        } else {
            data.push((None, block));
        }
        mode = !mode;
    }
    // print_disk(&data);
    for file_block in data
        .clone()
        .into_iter()
        .filter(|block| block.0.is_some())
        .rev()
    {
        // let DEBUG = file_block.0.unwrap();
        // true;
        let mut l = 0usize;
        let mut r = data
            .iter()
            .enumerate()
            .find_position(|block| *block.1 == file_block)
            .unwrap()
            .0;
        while l < r {
            if data[l].0.is_some() || data[l].1 < file_block.1 {
                l += 1;
                continue;
            }
            let diff = data[l].1 - file_block.1;
            if diff > 0 {
                if data[l + 1].0.is_none() {
                    data[l + 1].1 += diff;
                } else {
                    data[l].1 = diff;
                }
            } else {
                data.remove(l);
                r -= 1;
                // print_disk(&data);
            }
            let buf = data.remove(r);
            data.insert(l, buf);
            data.insert(r + 1, (None, buf.1));
            // print_disk(&data);
            break;
        }
    }
    // println!("{:?}", data);
    let answer = get_checksum_v2(data);
    println!("Part 2: {}", answer);
}

fn get_checksum(data: Vec<Option<usize>>) -> usize {
    data.iter()
        .enumerate()
        .fold(0, |acc, (index, id)| acc + (index * id.unwrap()))
}

fn print_disk(data: &Vec<(Option<usize>, usize)>) {
    for block in data {
        if block.0.is_some() {
            for _ in 0..block.1 {
                print!("{}", block.0.unwrap())
            }
        } else {
            for _ in 0..block.1 {
                print!(".")
            }
        }
    }
    println!()
}

fn get_checksum_v2(data: Vec<(Option<usize>, usize)>) -> usize {
    let mut checksum = 0;
    let mut index = 0;
    for block in data {
        if block.0.is_some() {
            for _ in 0..block.1 {
                checksum += block.0.unwrap() * index;
                index += 1;
            }
        } else {
            index += block.1;
        }
    }
    checksum
}
