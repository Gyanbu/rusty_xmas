use rusty_xmas;

fn main() {
    let input = rusty_xmas::load_input!();

    let line_length = input.lines().next().unwrap().len();
    let mut gamma_rate: u32 = 0;
    for i in 0..line_length {
        let acc: i32 = input.lines().map(|bits| bits.chars().skip(i).next().unwrap()).fold(0, |acc, bit| {
            if bit == '1' {
                acc + 1
            } else {
                acc - 1
            }
        });
        gamma_rate <<= 1;
        if acc > 0 {
            gamma_rate |= 1;
        } else {
            gamma_rate |= 0
        }
    }
    println!("Gamma rate: {}", gamma_rate);
    let xor_mask = (1 << line_length) - 1;
    let epsilon_rate = gamma_rate ^ xor_mask;
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Part 1: {}", gamma_rate * epsilon_rate);


    let line_length = input.lines().next().unwrap().len();

    let mut oxygen_generator_rating: u32 = 0;
    let mut input_copy: Vec<&str> = input.lines().collect();
    'outer: loop {
        for i in 0..line_length {
            if input_copy.len() == 1 {
                let val = input_copy[0];
                for bit in val.chars() {
                    oxygen_generator_rating <<= 1;
                    if bit == '1' {
                        oxygen_generator_rating |= 1;
                    } else {
                        oxygen_generator_rating |= 0;
                    }
                }
                break 'outer;
            }
            let count_ones = input_copy.iter().filter(|bits| bits.chars().nth(i) == Some('1')).count();
            if count_ones >= (input_copy.len() + 1) / 2 {
                input_copy = input_copy.into_iter().filter(|bits| bits.chars().nth(i) == Some('1')).collect();
            } else {
                input_copy = input_copy.into_iter().filter(|bits| bits.chars().nth(i) == Some('0')).collect();
            }
        }
    }
    println!("Oxygen_generator_rating: {}", oxygen_generator_rating);

    let mut co2_scrubber_rating: u32 = 0;
    let mut input_copy: Vec<&str> = input.lines().collect();
    'outer: loop {
        for i in 0..line_length {
            if input_copy.len() == 1 {
                let val = input_copy[0];
                for bit in val.chars() {
                    co2_scrubber_rating <<= 1;
                    if bit == '1' {
                        co2_scrubber_rating |= 1;
                    } else {
                        co2_scrubber_rating |= 0;
                    }
                }
                break 'outer;
            }
            let count_ones = input_copy.iter().filter(|bits| bits.chars().nth(i) == Some('1')).count();
            if count_ones < (input_copy.len() + 1) / 2 {
                input_copy = input_copy.into_iter().filter(|bits| bits.chars().nth(i) == Some('1')).collect();
            } else {
                input_copy = input_copy.into_iter().filter(|bits| bits.chars().nth(i) == Some('0')).collect();
            }
        }
    }
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);
    println!("Part 2: {}", oxygen_generator_rating * co2_scrubber_rating);
}