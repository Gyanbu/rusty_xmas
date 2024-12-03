use std::char;

fn main() {
    let input = rusty_xmas::load_input!();

    const MUL: &[char] = &['m', 'u', 'l', '('];
    let mut matching: usize = 0;
    let mut left_buf: Vec<u32> = Vec::new();
    let mut right_buf: Vec<u32> = Vec::new();
    let mut answer = 0;
    for c in input.chars() {
        match matching {
            0..=3 => {
                if c == MUL[matching] {
                    matching += 1;
                } else {
                    matching = 0;
                }
            }
            4 => {
                if c == ',' {
                    matching += 1;
                    continue;
                }
                match c.to_digit(10) {
                    Some(digit) => {
                        left_buf.push(digit);
                    }
                    None => {
                        left_buf.clear();
                        matching = 0
                    }
                }
            }
            5 => {
                if c == ')' {
                    let left_num = vec_to_u32(&mut left_buf);
                    let right_num = vec_to_u32(&mut right_buf);
                    answer += left_num * right_num;
                    matching = 0;
                    continue;
                }
                match c.to_digit(10) {
                    Some(digit) => {
                        right_buf.push(digit);
                    }
                    None => {
                        left_buf.clear();
                        right_buf.clear();
                        matching = 0
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    println!("Part 1: {}", answer);
}

fn vec_to_u32(vec: &mut Vec<u32>) -> u32 {
    let mut m = 1;
    let mut result = 0;
    for num in vec.drain(..).rev() {
        result += num * m;
        m *= 10;
    }
    result
}