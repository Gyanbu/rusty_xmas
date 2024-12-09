fn main() {
    let input = rusty_xmas::load_input!();
    let input: Vec<usize> = input.trim_end().chars().map(|num| num.to_digit(10).unwrap() as usize).collect();
    let mut data: Vec<Option<usize>> = Vec::new();
    
    let mut mode = true;
    let mut file_index: usize = 0;
    for block in input {
        if mode {
            data.append(&mut vec![Some(file_index); block]);
            file_index += 1;
        } else {
            data.append(&mut vec![None; block]);
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
    let answer = data.iter().enumerate().fold(0, |acc, (index, id)| {
        acc + (index * id.unwrap())
    });
    println!("Part 1: {}", answer);
}