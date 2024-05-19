use rusty_xmas;

fn main() {
    let input = rusty_xmas::load_input!();
    let mut data = input.lines();

    let mut previous_measurement: u32 = data.next().unwrap().parse().unwrap();
    let mut increments: u32 = 0;
    for line in data {
        if previous_measurement < line.parse().unwrap() {
            increments += 1;
        }
        previous_measurement = line.parse().unwrap();
    }
    println!("Part 1: {}", increments);

    let data: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut previous_windows_sum: u32 = data.iter().take(3).sum();
    let mut increments: u32 = 0;
    for i in 1..=data.len() - 3 {
        let window_sum = data[i] + data[i + 1] + data[i + 2];
        if window_sum > previous_windows_sum {
            increments += 1;
        }
        previous_windows_sum = window_sum;
    }
    println!("Part 2: {}", increments);
}
