use std::cmp::Ordering;

const MAX_STEP_SIZE: u32 = 3;

fn main() {
    let input = rusty_xmas::load_input!();

    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|report| {
            report
                .split_ascii_whitespace()
                .map(|reading| reading.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // dbg!(&reports);

    let mut safe_readings: usize = 0;
    'reports: for report in &reports {
        let ordering = report[0].cmp(&report[1]);
        if ordering.is_eq() {
            continue;
        }
        for readings in report.windows(2) {
            match ordering {
                Ordering::Less => {
                    if readings[1] <= readings[0] || MAX_STEP_SIZE < readings[1] - readings[0] {
                        continue 'reports;
                    }
                }
                Ordering::Greater => {
                    if readings[0] <= readings[1] || MAX_STEP_SIZE < readings[0] - readings[1] {
                        continue 'reports;
                    }
                }
                Ordering::Equal => unreachable!(),
            }
        }
        safe_readings += 1;
    }
    println!("Part 1: {}", safe_readings);

    let mut safe_readings: usize = 0;
    'reports: for report in &reports {
        'report: for i in (0..report.len() + 1).rev() {
            let mut report = report.clone();
            if i < report.len() {
                report.remove(i);
            }
            let ordering = report[0].cmp(&report[1]);
            if ordering.is_eq() {
                continue 'report;
            }
            for readings in report.windows(2) {
                match ordering {
                    Ordering::Less => {
                        if readings[1] <= readings[0] || MAX_STEP_SIZE < readings[1] - readings[0] {
                            continue 'report;
                        }
                    }
                    Ordering::Greater => {
                        if readings[0] <= readings[1] || MAX_STEP_SIZE < readings[0] - readings[1] {
                            continue 'report;
                        }
                    }
                    Ordering::Equal => unreachable!(),
                }
            }
            safe_readings += 1;
            continue 'reports;
        }
    }
    println!("Part 2: {}", safe_readings);
}
