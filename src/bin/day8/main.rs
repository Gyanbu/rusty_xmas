use rusty_xmas;

fn main() {
    let input = rusty_xmas::load_input!();

    let notes: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            line.split(" | ").collect::<Vec<&str>>()[1]
                .split(' ')
                .collect()
        })
        .collect();

    let mut answer: u32 = 0;
    for note in notes {
        for entry in note {
            if [2, 3, 4, 7].contains(&entry.len()) {
                answer += 1;
            }
        }
    }
    println!("Part 1: {}", answer);

    let notes: Vec<Vec<Vec<&str>>> = input
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|line| line.split(' ').collect())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect();

    let mut answer: u32 = 0;
    for mut note in notes {
        // println!("{:?} | {:?}", note[0], note[1]);
        let mut digits: [&str; 10] = [""; 10];

        for i in (0..note[0].len()).rev() {
            match note[0][i].len() {
                2 => {
                    digits[1] = note[0].remove(i);
                }
                3 => {
                    digits[7] = note[0].remove(i);
                }
                4 => {
                    digits[4] = note[0].remove(i);
                }
                7 => {
                    digits[8] = note[0].remove(i);
                }
                _ => continue,
            }
        }

        for i in (0..note[0].len()).rev() {
            match note[0][i].len() {
                5 => {
                    let mut cross_1: u32 = 0;
                    let mut cross_4: u32 = 0;
                    for segment in note[0][i].chars() {
                        if digits[4].contains(segment) {
                            cross_4 += 1;
                        }
                        if digits[1].contains(segment) {
                            cross_1 += 1;
                        }
                    }
                    if cross_1 == 2 {
                        digits[3] = note[0].remove(i);
                    } else if cross_4 == 3 {
                        digits[5] = note[0].remove(i);
                    } else {
                        digits[2] = note[0].remove(i);
                    }
                }
                6 => {
                    let mut cross_4: u32 = 0;
                    let mut cross_7: u32 = 0;
                    for segment in note[0][i].chars() {
                        if digits[4].contains(segment) {
                            cross_4 += 1;
                        }
                        if digits[7].contains(segment) {
                            cross_7 += 1;
                        }
                    }
                    if cross_4 == 4 {
                        digits[9] = note[0].remove(i);
                    } else if cross_7 == 2 {
                        digits[6] = note[0].remove(i);
                    } else {
                        digits[0] = note[0].remove(i);
                    }
                }
                _ => panic!(),
            }
        }
        // println!("0 -> {}\n1 -> {}\n2 -> {}\n3 -> {}\n4 -> {}\n5 -> {}\n6 -> {}\n7 -> {}\n8 -> {}\n9 -> {}", digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6], digits[7], digits[8], digits[9]);
        // println!("{digits:?}");

        let mut sorted_digits: Vec<String> = Vec::new();
        for digit in digits.iter() {
            let mut digit: Vec<char> = digit.chars().collect();
            digit.sort();
            let digit: String = digit.into_iter().collect();
            sorted_digits.push(digit);
        }
        // println!("{sorted_digits:?}");

        for (exp, digit) in note[1].iter().rev().enumerate() {
            let mut digit: Vec<char> = digit.chars().collect();
            digit.sort();
            let digit: String = digit.into_iter().collect();
            for i in 0..sorted_digits.len() {
                if digit == sorted_digits[i] {
                    answer += i as u32 * 10u32.pow(exp as u32);
                }
            }
        }
    }
    println!("Part 2: {}", answer);
}
